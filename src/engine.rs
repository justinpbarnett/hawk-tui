use crate::{
    clipboard::Clipboard,
    config::Options,
    diff::{DiffLine, LineKind, RepoDiff},
    diff_loader::{DiffLoadRequest, DiffLoader},
    document::{ReviewDocument, ReviewRow},
    export::PromptExporter,
    git::{CliGit, GitAdapter},
    prompt::CopyScope,
    session::{LineAnchor, Session},
    session_store::SessionStore,
    workspace,
};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct ReviewEngine {
    pub opts: Options,
    pub repos: Vec<RepoDiff>,
    pub document: ReviewDocument,
    pub session: Session,
    pub dirty: bool,
    pub(crate) fingerprint: String,
}
impl ReviewEngine {
    pub fn open(opts: Options) -> Self {
        Self::open_with_git(opts, &CliGit)
    }
    pub fn open_with_git<G: GitAdapter>(opts: Options, git: &G) -> Self {
        let root = opts.cwd.canonicalize().unwrap_or(opts.cwd.clone());
        let mut e = Self {
            session: Session::load(&root),
            opts,
            repos: vec![],
            document: ReviewDocument::default(),
            dirty: false,
            fingerprint: String::new(),
        };
        e.reload_with_git(git);
        e
    }
    pub fn reload(&mut self) {
        self.reload_with_git(&CliGit);
    }
    pub fn reload_with_git<G: GitAdapter>(&mut self, git: &G) {
        let mut targets = workspace::discover_with_git(
            git,
            &self.opts.cwd,
            &self.opts.config,
            self.opts.force_repo,
            self.opts.force_workspace,
        );
        if targets.is_empty() && !matches!(self.opts.mode, crate::config::ReviewMode::Default) {
            if let Some(root) = git.repo_root(&self.opts.cwd) {
                targets.push(workspace::RepoTarget {
                    display: root
                        .strip_prefix(&self.opts.cwd)
                        .unwrap_or(&root)
                        .to_string_lossy()
                        .trim_start_matches('/')
                        .to_string(),
                    root,
                    nested_children: vec![],
                });
            }
        }
        let mut repos = Vec::new();
        for t in targets {
            repos.push(DiffLoader::load(
                git,
                DiffLoadRequest {
                    repo: &t.root,
                    display: if t.display.is_empty() {
                        ".".into()
                    } else {
                        t.display
                    },
                    mode: &self.opts.mode,
                    config: &self.opts.config,
                    nested: &t.nested_children,
                },
            ));
        }
        self.reconcile(&repos);
        self.repos = repos;
        self.document = ReviewDocument::from_repos(&self.repos, &self.session);
        self.fingerprint = self.compute_fingerprint_with_git(git);
        self.dirty = false;
    }
    pub fn poll_dirty(&mut self) -> bool {
        self.poll_dirty_with_git(&CliGit)
    }
    pub fn poll_dirty_with_git<G: GitAdapter>(&mut self, git: &G) -> bool {
        let now = self.compute_fingerprint_with_git(git);
        self.dirty = now != self.fingerprint;
        self.dirty
    }
    fn compute_fingerprint_with_git<G: GitAdapter>(&self, git: &G) -> String {
        let mut h = Sha256::new();
        for r in &self.repos {
            h.update(git.dirty_fingerprint_input(&r.repo_path));
        }
        format!("{:x}", h.finalize())
    }
    fn reconcile(&mut self, new_repos: &[RepoDiff]) {
        let doc = ReviewDocument::from_repos(new_repos, &self.session);
        let anchors: BTreeSet<_> = doc
            .rows()
            .iter()
            .filter_map(crate::document::anchor_from_row)
            .map(|a| a.key())
            .collect();
        let root = self
            .opts
            .cwd
            .canonicalize()
            .unwrap_or(self.opts.cwd.clone());
        let mut store = SessionStore {
            root,
            session: self.session.clone(),
            autosaves: 0,
        };
        store.reconcile_missing(&anchors);
        self.session = store.session;
    }
    pub fn anchor_for_row(&self, idx: usize) -> Option<LineAnchor> {
        self.document.anchor_at(idx)
    }
    pub fn comment(&mut self, idx: usize, body: String) -> String {
        if let Some(a) = self.anchor_for_row(idx) {
            let root = self.root();
            let mut store = SessionStore {
                root,
                session: self.session.clone(),
                autosaves: 0,
            };
            store.upsert_comment(a, body);
            self.session = store.session;
            self.refresh_document();
            "comment saved".into()
        } else {
            "comments attach only to diff content lines".into()
        }
    }
    pub fn delete_comment(&mut self, idx: usize) {
        if let Some(a) = self.anchor_for_row(idx) {
            let root = self.root();
            let mut store = SessionStore {
                root,
                session: self.session.clone(),
                autosaves: 0,
            };
            store.delete_anchor(&a);
            self.session = store.session;
            self.refresh_document();
        }
    }
    pub fn delete_all_visible_comments(&mut self) {
        let root = self.root();
        let mut store = SessionStore {
            root,
            session: self.session.clone(),
            autosaves: 0,
        };
        store.delete_all_visible();
        self.session = store.session;
        self.refresh_document();
    }
    pub fn toggle_resolved(&mut self, idx: usize) {
        if let Some(a) = self.anchor_for_row(idx) {
            let root = self.root();
            let mut store = SessionStore {
                root,
                session: self.session.clone(),
                autosaves: 0,
            };
            store.toggle_resolved(&a);
            self.session = store.session;
            self.refresh_document();
        }
    }
    pub fn toggle_show_resolved(&mut self) {
        let root = self.root();
        let mut store = SessionStore {
            root,
            session: self.session.clone(),
            autosaves: 0,
        };
        store.set_show_resolved(!store.session.show_resolved);
        self.session = store.session;
        self.refresh_document();
    }
    pub fn clear_resolved(&mut self) {
        let root = self.root();
        let mut store = SessionStore {
            root,
            session: self.session.clone(),
            autosaves: 0,
        };
        store.clear_resolved();
        self.session = store.session;
        self.refresh_document();
    }
    pub fn reset_session(&mut self) {
        let mut store = SessionStore::load(self.root());
        store.reset();
        self.session = store.session;
        self.refresh_document();
    }
    pub fn export<C: Clipboard>(&mut self, scope: CopyScope, cb: &mut C) -> String {
        let root = self.root();
        let mut store = SessionStore {
            root,
            session: self.session.clone(),
            autosaves: 0,
        };
        let status = PromptExporter::export(&mut store, scope, cb).to_string();
        self.session = store.session;
        self.refresh_document();
        status
    }
    pub fn has_uncopied_drafts(&self) -> bool {
        self.session
            .comments
            .values()
            .any(|c| c.status == crate::session::CommentStatus::Draft)
    }
    pub fn autosave(&self) {
        let _ = self.session.save(&self.root());
    }
    fn root(&self) -> std::path::PathBuf {
        self.opts
            .cwd
            .canonicalize()
            .unwrap_or(self.opts.cwd.clone())
    }
    fn refresh_document(&mut self) {
        self.document = ReviewDocument::from_repos(&self.repos, &self.session);
    }
}

pub use crate::document::{flatten, line_is_content};

#[allow(dead_code)]
fn _keep_imports(_: ReviewRow, _: DiffLine, _: LineKind) {}
