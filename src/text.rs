use crate::config::Config;
use std::{fs, path::Path};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextStatus {
    Text,
    Huge { reason: String },
    Binary(String),
    TooLarge(String),
}
const NON_TEXT: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "webp", "zip", "gz", "tar", "woff", "ttf", "sqlite", "db", "pdf",
    "mp4", "mov", "ico",
];
pub fn classify_path(path: &Path, cfg: &Config) -> TextStatus {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if NON_TEXT.contains(&ext.as_str()) {
        return TextStatus::Binary(format!("known non-text extension .{ext}"));
    }
    let meta = match fs::metadata(path) {
        Ok(m) => m,
        Err(e) => return TextStatus::Binary(e.to_string()),
    };
    if meta.len() > cfg.absolute_file_size {
        return TextStatus::TooLarge("over absolute size limit".into());
    }
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => return TextStatus::Binary(e.to_string()),
    };
    if bytes.contains(&0) {
        return TextStatus::Binary("contains NUL byte".into());
    }
    if meta.len() > cfg.eager_file_size {
        return TextStatus::Huge {
            reason: "over eager file-size threshold".into(),
        };
    };
    if std::str::from_utf8(&bytes).is_ok() {
        TextStatus::Text
    } else {
        let bad = bytes
            .iter()
            .filter(|b| **b < 0x09 || (**b > 0x0d && **b < 0x20))
            .count();
        if bad * 10 < bytes.len().max(1) {
            TextStatus::Text
        } else {
            TextStatus::Binary("not mostly valid text".into())
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binary_and_lockfile() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("a.png");
        fs::write(&p, b"x").unwrap();
        assert!(matches!(
            classify_path(&p, &Config::default()),
            TextStatus::Binary(_)
        ));
        let l = d.path().join("Cargo.lock");
        fs::write(&l, "text").unwrap();
        assert_eq!(classify_path(&l, &Config::default()), TextStatus::Text);
    }
}
