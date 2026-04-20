#[inline(always)]
pub fn is_empty_or_none<T>(vec: &Option<Vec<T>>) -> bool {
    if let Some(vec) = vec {
        return vec.is_empty();
    }
    true
}
