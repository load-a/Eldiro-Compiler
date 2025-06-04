// #[derive(Debug, PartialEq)]

pub(crate) fn tag<'a, 'b>(keyword: &'a str, source: &'b str) -> &'b str {
    if source.starts_with(keyword) {
        &source[keyword.len()..]
    } else {
        panic!("Expected {}", keyword)
    }
}

pub(crate) fn extract_digits(source: &str) -> (&str, &str) {
    take_while(|character| character.is_ascii_digit(), source)
}

pub(crate) fn extract_operation(source: &str) -> (&str, &str) {
    match &source[0..1] {
        "+" | "-" | "*" | "/" => {},
        _ => panic!("Bad operator"),
    }

    (&source[1..], &source[0..1])
}

pub(crate) fn extract_whitespace(source: &str) -> (&str, &str) {
    take_while(|character| character == ' ', source)
}

pub(crate) fn extract_identitifier(source: &str) -> (&str, &str) {
    let input_start_is_alphanumeric = source
        .chars()
        .next()
        .map(|character| character.is_ascii_alphabetic())
        .unwrap_or(false);

    if input_start_is_alphanumeric {
        take_while(|character| character.is_ascii_alphanumeric(), source)
    } else {
        (source, "")
    }
}

fn take_while(accept: impl Fn(char) -> bool, source: &str) -> (&str, &str) {
    let extracted_end = source
            .char_indices()
            .find_map(|(index, character)| if accept(character) { None } else { Some(index) })
            .unwrap_or_else(|| source.len());

    let extracted = &source[0..extracted_end];
    let remainder = &source[extracted_end..];

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

    #[test]
    fn cannot_extract_numeric_identifier() {
        assert_eq!(extract_identitifier("123abc"), ("123abc", ""));
    }

    #[test]
    fn tag_keyword() {
        assert_eq!(tag("let", "let a = 5"), " a = 5")
    }
}