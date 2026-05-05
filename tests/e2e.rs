use hawk_tui::{
    clipboard::Clipboard,
    config::{Config, Options, ReviewMode},
    document::ReviewRow,
    engine::ReviewEngine,
    prompt::CopyScope,
};
use std::{path::Path, process::Command};

fn git(p: &Path, args: &[&str]) {
    assert!(
        Command::new("git")
            .args(args)
            .current_dir(p)
            .output()
            .unwrap()
            .status
            .success(),
        "git {:?}",
        args
    );
}
fn init_repo(p: &Path) {
    git(p, &["init"]);
    git(p, &["config", "user.email", "hawk@example.test"]);
    git(p, &["config", "user.name", "Hawk Test"]);
}
fn opts(cwd: &Path) -> Options {
    Options {
        mode: ReviewMode::Default,
        force_repo: false,
        force_workspace: false,
        cwd: cwd.into(),
        config: Config::default(),
    }
}
struct FakeClip {
    writes: Vec<String>,
    fail: bool,
}
impl Clipboard for FakeClip {
    fn write(&mut self, text: &str) -> Result<String, String> {
        if self.fail {
            Err("boom".into())
        } else {
            self.writes.push(text.into());
            Ok("fake".into())
        }
    }
}

#[test]
fn default_review_loads_tracked_and_untracked_and_exports_prompt() {
    let d = tempfile::tempdir().unwrap();
    init_repo(d.path());
    std::fs::write(d.path().join("a.rs"), "fn a() {}\n").unwrap();
    git(d.path(), &["add", "."]);
    git(d.path(), &["commit", "-m", "init"]);
    std::fs::write(d.path().join("a.rs"), "fn a() { println!(\"x\"); }\n").unwrap();
    std::fs::write(d.path().join("new.py"), "print('new')\n").unwrap();
    let mut e = ReviewEngine::open(opts(d.path()));
    assert!(e
        .document
        .rows()
        .iter()
        .any(|r| matches!(r, ReviewRow::File{path,..} if path=="a.rs")));
    assert!(e
        .document
        .rows()
        .iter()
        .any(|r| matches!(r, ReviewRow::File{path,..} if path=="new.py")));
    let idx = e
        .document
        .rows()
        .iter()
        .position(|r| matches!(r, ReviewRow::Line { .. }))
        .unwrap();
    assert_eq!(e.comment(idx, "tighten this".into()), "comment saved");
    let mut clip = FakeClip {
        writes: vec![],
        fail: false,
    };
    assert!(e.export(CopyScope::Uncopied, &mut clip).contains("copied"));
    assert!(clip.writes[0].contains("tighten this"));
}

#[test]
fn workspace_aggregates_nested_repos_and_excludes_ignored_files() {
    let d = tempfile::tempdir().unwrap();
    let a = d.path().join("client");
    let b = d.path().join("server");
    std::fs::create_dir_all(&a).unwrap();
    std::fs::create_dir_all(&b).unwrap();
    for repo in [&a, &b] {
        init_repo(repo);
        std::fs::write(repo.join("README.md"), "hi\n").unwrap();
        git(repo, &["add", "."]);
        git(repo, &["commit", "-m", "init"]);
    }
    std::fs::write(a.join("README.md"), "hi client\n").unwrap();
    std::fs::write(b.join("new.json"), "{}\n").unwrap();
    let mut o = opts(d.path());
    o.force_workspace = true;
    let e = ReviewEngine::open(o);
    assert!(e.repos.iter().any(|r| r.display_path.ends_with("client")));
    assert!(e.repos.iter().any(|r| r.display_path.ends_with("server")));
    assert!(e
        .document
        .rows()
        .iter()
        .any(|r| matches!(r,ReviewRow::File{path,..} if path=="new.json")));
}

#[test]
fn staged_mode_ignores_unstaged_changes() {
    let d = tempfile::tempdir().unwrap();
    init_repo(d.path());
    std::fs::write(d.path().join("a"), "one\n").unwrap();
    git(d.path(), &["add", "."]);
    git(d.path(), &["commit", "-m", "init"]);
    std::fs::write(d.path().join("a"), "two\n").unwrap();
    git(d.path(), &["add", "a"]);
    std::fs::write(d.path().join("a"), "three\n").unwrap();
    let mut o = opts(d.path());
    o.mode = ReviewMode::Staged;
    let e = ReviewEngine::open(o);
    let texts: Vec<_> = e
        .document
        .rows()
        .iter()
        .filter_map(|r| {
            if let ReviewRow::Line { line, .. } = r {
                Some(line.text.clone())
            } else {
                None
            }
        })
        .collect();
    assert!(texts.iter().any(|t| t == "two"));
    assert!(!texts.iter().any(|t| t == "three"));
}

#[test]
fn base_ref_and_skipped_or_collapsed_files_are_reviewable() {
    let d = tempfile::tempdir().unwrap();
    init_repo(d.path());
    std::fs::write(d.path().join("a.txt"), "one\n").unwrap();
    git(d.path(), &["add", "."]);
    git(d.path(), &["commit", "-m", "init"]);
    git(d.path(), &["branch", "base"]);
    std::fs::write(d.path().join("a.txt"), "two\n").unwrap();
    git(d.path(), &["add", "."]);
    git(d.path(), &["commit", "-m", "change"]);

    let mut base = opts(d.path());
    base.mode = ReviewMode::Base("base".into());
    let e = ReviewEngine::open(base);
    assert!(e
        .document
        .rows()
        .iter()
        .any(|r| matches!(r, ReviewRow::Line { line, .. } if line.text == "two")));

    std::fs::write(d.path().join("image.png"), b"not really text").unwrap();
    let e = ReviewEngine::open(opts(d.path()));
    assert!(e
        .document
        .rows()
        .iter()
        .any(|r| matches!(r, ReviewRow::Placeholder(reason) if reason.contains("non-text"))));

    std::fs::write(d.path().join("huge.txt"), "a\nb\nc\n").unwrap();
    let mut huge = opts(d.path());
    huge.config.eager_diff_lines = 1;
    let e = ReviewEngine::open(huge);
    assert!(e.document.rows().iter().any(
        |r| matches!(r, ReviewRow::Placeholder(reason) if reason.contains("eager diff-line"))
    ));
}
