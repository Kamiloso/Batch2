pub fn len(s: &str) -> usize {
    s.chars().count()
}

pub fn truncate(s: &str, len: usize) -> &str {
    if let Some((idx, _)) = s.char_indices().nth(len) {
        &s[..idx]
    } else {
        s
    }
}