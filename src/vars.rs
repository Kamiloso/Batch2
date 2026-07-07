const DELIM_VAR: char = '%';
const DELIM_CONST: char = '!';

pub fn perform_expansion<'a>(
    value: impl AsRef<str>,
    mut get: impl FnMut(&str) -> &'a str,
    mut get_const: impl FnMut(&str) -> &'a str,
) -> String {
    let source_str = value.as_ref();
    let mut result = String::with_capacity(source_str.len());
    let mut slice = source_str;

    while let Some(idx) = slice.find(&[DELIM_VAR, DELIM_CONST]) {
        result.push_str(&slice[..idx]);

        let delim = slice[idx..].chars().next().unwrap();
        let after_delim = &slice[idx + delim.len_utf8()..];
        
        slice = match after_delim.find(delim) {
            Some(end_offset) => {
                let name = &after_delim[..end_offset];

                if name.is_empty() {
                    result.push(delim);
                } else {
                    if delim == DELIM_VAR {
                        result.push_str(get(name));
                    } else {
                        result.push_str(get_const(name));
                    }
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