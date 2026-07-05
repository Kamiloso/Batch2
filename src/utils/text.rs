use std::ops::Range;

pub fn strlen(s: &str) -> usize {
    s.chars().count()
}

pub fn substr(s: &str, range: Range<usize>) -> &str {
    if range.start >= range.end {
        return "";
    }

    let mut iter = s.char_indices();

    let Some((start, _)) = iter.nth(range.start) else {
        return "";
    };
    
    let end = match iter.nth(range.end - range.start - 1) {
        Some((end, _)) => end,
        None => s.len()
    };
    
    &s[start..end]
}