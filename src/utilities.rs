// #[derive(Debug, PartialEq)]
pub(crate) fn extract_digits(slice: &str) -> (&str, &str) {
    take_while(|character| character.is_ascii_digit(), slice)
}

pub(crate) fn extract_operation(slice: &str) -> (&str, &str) {
    match &slice[0..1] {
        "+" | "-" | "*" | "/" => {},
        _ => panic!("Bad operator"),
    }

    (&slice[1..], &slice[0..1])
}

pub(crate) fn extract_whitespace(slice: &str) -> (&str, &str) {
    take_while(|character| character == ' ', slice)
}

fn take_while(accept: impl Fn(char) -> bool, slice: &str) -> (&str, &str) {
    let extracted_end = slice
            .char_indices()
            .find_map(|(index, character)| if accept(character) { None } else { Some(index) })
            .unwrap_or_else(|| slice.len());

    let extracted = &slice[0..extracted_end];
    let remainder = &slice[extracted_end..];

    (remainder, extracted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
    }

    #[test]
    fn extract_multiple_digit() {
        assert_eq!(extract_digits("10+20"), ("+20", "10"));
    }

    #[test]
    fn extract_no_digits() {
        assert_eq!(extract_digits(""), ("", ""));
    }

    #[test]
    fn extract_only_digits() {
        assert_eq!(extract_digits("1000"), ("", "1000"));
    }

    #[test]
    fn extract_plus() {
        assert_eq!(extract_operation("+1234"), ("1234", "+"));
    }

    #[test]
    fn extract_minus() {
        assert_eq!(extract_operation("-1234"), ("1234", "-"));
    }

    #[test]
    fn extract_asterisk() {
        assert_eq!(extract_operation("*1234"), ("1234", "*"));
    }

    #[test]
    fn extract_slash() {
        assert_eq!(extract_operation("/1234"), ("1234", "/"));
    }

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespace("   abcd"), ("abcd", "   "));
    }
}