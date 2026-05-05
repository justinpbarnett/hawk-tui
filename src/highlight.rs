#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StyleKind {
    Plain,
    Add,
    Remove,
    Intraline,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub text: String,
    pub style: StyleKind,
}
pub fn language_for(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or("") {
        "rs" => "rust",
        "ts" => "typescript",
        "tsx" => "tsx",
        "js" => "javascript",
        "jsx" => "jsx",
        "py" => "python",
        "go" => "go",
        "php" => "php",
        "sql" => "sql",
        "json" => "json",
        "yaml" | "yml" => "yaml",
        "toml" => "toml",
        "md" => "markdown",
        "sh" | "bash" => "bash",
        "html" => "html",
        "css" => "css",
        _ => "plain",
    }
}
pub fn intraline_removed_added(old: &str, new: &str) -> Option<(Vec<Span>, Vec<Span>)> {
    if old.is_empty() || new.is_empty() {
        return None;
    }
    let pre = old
        .chars()
        .zip(new.chars())
        .take_while(|(a, b)| a == b)
        .count();
    let os: Vec<char> = old.chars().collect();
    let ns: Vec<char> = new.chars().collect();
    let mut suf = 0;
    while suf + pre < os.len()
        && suf + pre < ns.len()
        && os[os.len() - 1 - suf] == ns[ns.len() - 1 - suf]
    {
        suf += 1
    }
    if pre == 0 && suf == 0 {
        return None;
    }
    Some((
        mark(old, pre, suf, StyleKind::Remove),
        mark(new, pre, suf, StyleKind::Add),
    ))
}
fn mark(s: &str, pre: usize, suf: usize, base: StyleKind) -> Vec<Span> {
    let chars: Vec<char> = s.chars().collect();
    let mut v = Vec::new();
    if pre > 0 {
        v.push(Span {
            text: chars[..pre].iter().collect(),
            style: base.clone(),
        })
    }
    if chars.len() >= pre + suf {
        v.push(Span {
            text: chars[pre..chars.len() - suf].iter().collect(),
            style: StyleKind::Intraline,
        })
    }
    if suf > 0 {
        v.push(Span {
            text: chars[chars.len() - suf..].iter().collect(),
            style: base,
        })
    }
    v.into_iter().filter(|x| !x.text.is_empty()).collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn langs_and_spans() {
        assert_eq!(language_for("x.tsx"), "tsx");
        let (a, b) = intraline_removed_added("foo old bar", "foo new bar").unwrap();
        assert!(a.iter().any(|s| s.style == StyleKind::Intraline));
        assert!(b.iter().any(|s| s.text == "new"));
        assert!(intraline_removed_added("abc", "xyz").is_none());
    }
}
