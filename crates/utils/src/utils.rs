use std::ffi::OsStr;
use std::process::Command;

#[cfg(target_os = "macos")]
const EXPLORER_OPEN_CMD: &str = "open";
#[cfg(target_os = "windows")]
const EXPLORER_OPEN_CMD: &str = "explorer";
#[cfg(target_os = "linux")]
const EXPLORER_OPEN_CMD: &str = "xdb-open";

pub fn open_explorer<P: AsRef<OsStr>>(path: P) {
    Command::new(EXPLORER_OPEN_CMD).arg(path).spawn().unwrap();
}
