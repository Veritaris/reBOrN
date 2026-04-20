use std::ffi::OsStr;
use std::process::Command;

#[cfg(target_os = "macos")]
const EXPLORER_OPEN_CMD: &str = "open";
#[cfg(target_os = "windows")]
const EXPLORER_OPEN_CMD: &str = "explorer";
#[cfg(target_os = "linux")]
const EXPLORER_OPEN_CMD: &str = "xdb-open";

pub fn split_once<'a>(in_string: &'a str, sep: &str) -> Option<(&'a str, &'a str)> {
    let mut splitter = in_string.splitn(2, sep);
    let first = match splitter.next() {
        None => { return None; }
        Some(res) => { res }
    };
    let second = match splitter.next() {
        None => { return None; }
        Some(res) => { res }
    };
    Some((first, second))
}

pub fn open_explorer<P: AsRef<OsStr>>(path: P) {
    Command::new(EXPLORER_OPEN_CMD)
        .arg(path)
        .spawn()
        .unwrap();
}
