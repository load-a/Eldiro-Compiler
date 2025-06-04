// #[derive(Debug, PartialEq)]

pub(crate) fn tag<'a, 'b>(keyword: &'a str, source: &'b str) -> Result<&'b str, String> {
    if source.starts_with(keyword) {
        Ok(&source[keyword.len()..])
    } else {
        Err(format!("Expected {}", keyword))
    }
}

pub(crate) fn extract_digits(source: &str) -> Result<(&str, &str), String> {
    take_only(|character| character.is_ascii_digit(), source, "Expected digits".to_string())
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

pub(crate) fn require_whitespace(source: &str) -> Result<(&str, &str), String> {
    take_only(|character| character == ' ', source, "Expected whitespace".to_string())
}

pub(crate) fn extract_identitifier(source: &str) -> Result<(&str, &str), String> {
    let input_start_is_alphanumeric = source
        .chars()
        .next()
        .map(|character| character.is_ascii_alphabetic())
        .unwrap_or(false);

    if input_start_is_alphanumeric {
        Ok(take_while(|character| character.is_ascii_alphanumeric(), source))
    } else {
        Err("Expected identifier".to_string())
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

pub(crate) fn take_only(
    accept: impl Fn(char) -> bool, 
    source: &str, 
    error_message: String
) -> Result<(&str, &str), String> {
    let (remainder, extracted) = take_while(accept, source);

    if extracted.is_empty() {
        Err(error_message)
    } else {
        Ok((remainder, extracted))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), Ok(("+2", "1")));
    }

    #[test]
    fn extract_multiple_digit() {
        assert_eq!(extract_digits("10+20"), Ok(("+20", "10")));
    }

    #[test]
    fn extract_digits_from_invalid_input() {
        assert_eq!(extract_digits(""), Err("Expected digits".to_string()));
    }

    #[test]
    fn extract_only_digits() {
        assert_eq!(extract_digits("1000"), Ok(("", "1000")));
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
    fn extract_alphabetic_identifier() {
        assert_eq!(extract_identitifier("abc"), Ok(("", "abc")));
    }

    #[test]
    fn extract_alphanumeric_identifier() {
        assert_eq!(extract_identitifier("abc123"), Ok(("", "abc123")));
    }

    #[test]
    fn cannot_extract_numeric_identifier() {
        assert_eq!(extract_identitifier("123abc"), Err("Expected identifier".to_string()));
    }

    #[test]
    fn tag_keyword() {
        assert_eq!(tag("let", "let a = 5"), Ok(" a = 5"));
    }

    #[test]
    fn extract_required_whitespace() {
        assert_eq!(require_whitespace(""), Err("Expected whitespace".to_string()))
    }
}