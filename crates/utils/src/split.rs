pub fn split_once<'a>(in_string: &'a str, sep: &'a str) -> Option<(&'a str, &'a str)> {
    let mut splitter = in_string.splitn(2, sep);
    let first = match splitter.next() {
        None => {
            return None;
        }
        Some(res) => res,
    };
    let second = match splitter.next() {
        None => {
            return None;
        }
        Some(res) => res,
    };
    Some((first, second))
}

pub fn split_twice<'a>(in_string: &'a str, sep: &'a str) -> Option<(&'a str, &'a str, &'a str)> {
    let (first, second) = split_once(in_string, sep)?;
    let (second, third) = split_once(second, sep)?;
    Some((first, second, third))
}

pub fn split_thrice<'a>(in_string: &'a str, sep: &'a str) -> Option<(&'a str, &'a str, &'a str, &'a str)> {
    let (first, second) = split_once(in_string, sep)?;
    let (second, third) = split_once(second, sep)?;
    let (third, fourth) = split_once(third, sep)?;
    Some((first, second, third, fourth))
}

pub fn split_quice<'a>(in_string: &'a str, sep: &'a str) -> Option<(&'a str, &'a str, &'a str, &'a str, &'a str)> {
    let (first, second) = split_once(in_string, sep)?;
    let (second, third) = split_once(second, sep)?;
    let (third, fourth) = split_once(third, sep)?;
    let (fourth, fifth) = split_once(fourth, sep)?;
    Some((first, second, third, fourth, fifth))
}

pub fn split_once_maybe<'a>(in_string: &'a str, sep: &'a str) -> (Option<&'a str>, Option<&'a str>) {
    let mut splitter = in_string.splitn(2, sep);
    let first = match splitter.next() {
        None => {
            return (None, None);
        }
        Some(res) => Some(res),
    };
    let second = match splitter.next() {
        None => {
            return (first, None);
        }
        Some(res) => Some(res),
    };
    (first, second)
}

pub fn split_twice_maybe<'a>(in_string: &'a str, sep: &'a str) -> (Option<&'a str>, Option<&'a str>, Option<&'a str>) {
    let (first, second) = split_once_maybe(in_string, sep);
    if second.is_none() {
        return (first, second, None);
    }

    let (second, third) = split_once_maybe(second.expect("I have to be safe"), sep);

    (first, second, third)
}

pub fn split_thrice_maybe<'a>(
    in_string: &'a str,
    sep: &'a str,
) -> (Option<&'a str>, Option<&'a str>, Option<&'a str>, Option<&'a str>) {
    let (first, second) = split_once_maybe(in_string, sep);
    if second.is_none() {
        return (first, second, None, None);
    }
    let (second, third) = split_once_maybe(second.expect("I have to be safe"), sep);
    if third.is_none() {
        return (first, second, third, None);
    }
    let (third, fourth) = split_once_maybe(third.expect("I have to be safe"), sep);
    (first, second, third, fourth)
}

pub fn split_quice_maybe<'a>(
    in_string: &'a str,
    sep: &'a str,
) -> (
    Option<&'a str>,
    Option<&'a str>,
    Option<&'a str>,
    Option<&'a str>,
    Option<&'a str>,
) {
    let (first, second) = split_once_maybe(in_string, sep);
    if second.is_none() {
        return (first, second, None, None, None);
    }
    let (second, third) = split_once_maybe(second.expect("I have to be safe"), sep);
    if third.is_none() {
        return (first, second, third, None, None);
    }
    let (third, fourth) = split_once_maybe(third.expect("I have to be safe"), sep);
    if fourth.is_none() {
        return (first, second, third, fourth, None);
    }
    let (fourth, fifth) = split_once_maybe(fourth.expect("I have to be safe"), sep);
    (first, second, third, fourth, fifth)
}
