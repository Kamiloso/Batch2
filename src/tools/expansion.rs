use crate::tools::chars;

pub fn perform_expansion<'a>(
    value: impl AsRef<str>,
    mut get: impl FnMut(&str, char) -> &'a str,
) -> String {
    let source_str = value.as_ref();
    let mut result = String::with_capacity(source_str.len());
    let mut slice = source_str;

    while let Some(idx) = slice.find(chars::is_expansion) {
        result.push_str(&slice[..idx]);

        let delim = slice[idx..].chars().next().unwrap();
        let after_delim = &slice[idx + delim.len_utf8()..];
        
        slice = match after_delim.find(delim) {
            Some(end_offset) => {
                let name = &after_delim[..end_offset];

                if name.is_empty() {
                    result.push(delim);
                } else {
                    result.push_str(get(name, delim));
                }

                &after_delim[end_offset + delim.len_utf8()..]
            },
            None => {
                result.push(delim);
                after_delim
            },
        };
    }
    
    result.push_str(slice);
    result
}

pub fn duplicate_expansion_chars(value: impl AsRef<str>) -> String {
    let source_str = value.as_ref();
    let mut result = String::with_capacity(source_str.len());

    for c in source_str.chars() {
        result.push(c);
        
        if chars::is_expansion(c) {
            result.push(c);
        }
    }

    result
}