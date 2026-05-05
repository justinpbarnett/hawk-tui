use std::{fs, process::Command};
pub trait Clipboard {
    fn write(&mut self, text: &str) -> Result<String, String>;
}
pub struct FallbackClipboard;
impl Clipboard for FallbackClipboard {
    fn write(&mut self, text: &str) -> Result<String, String> {
        export(text)
    }
}
pub fn export(text: &str) -> Result<String, String> {
    if try_cmd("pbcopy", text)
        .or_else(|_| try_cmd("wl-copy", text))
        .or_else(|_| try_cmd("xclip", text))
        .is_ok()
    {
        return Ok("system clipboard".into());
    }
    if std::env::var("TERM").is_ok() {
        print!("\x1b]52;c;{}\x07", base64_simple(text.as_bytes()));
        return Ok("osc52".into());
    }
    let mut p = std::env::temp_dir();
    p.push("hawk-review-prompt.md");
    fs::write(&p, text).map_err(|e| e.to_string())?;
    Ok(format!("temp file {}", p.display()))
}
fn try_cmd(cmd: &str, text: &str) -> Result<(), String> {
    let mut child = Command::new(cmd)
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    use std::io::Write;
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(text.as_bytes())
        .map_err(|e| e.to_string())?;
    if child.wait().map_err(|e| e.to_string())?.success() {
        Ok(())
    } else {
        Err("failed".into())
    }
}
fn base64_simple(b: &[u8]) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut o = String::new();
    for c in b.chunks(3) {
        let n = ((c[0] as u32) << 16)
            | ((*c.get(1).unwrap_or(&0) as u32) << 8)
            | (*c.get(2).unwrap_or(&0) as u32);
        o.push(T[((n >> 18) & 63) as usize] as char);
        o.push(T[((n >> 12) & 63) as usize] as char);
        if c.len() > 1 {
            o.push(T[((n >> 6) & 63) as usize] as char)
        } else {
            o.push('=')
        }
        if c.len() > 2 {
            o.push(T[(n & 63) as usize] as char)
        } else {
            o.push('=')
        }
    }
    o
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base64() {
        assert_eq!(base64_simple(b"hi"), "aGk=");
    }
}
