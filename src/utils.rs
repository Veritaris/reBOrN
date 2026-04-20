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