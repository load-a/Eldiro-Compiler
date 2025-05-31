// #[derive(Debug, PartialEq)]
pub(crate) fn extract_digits(slice: &str) -> (&str, &str) {
    let digit_end = slice
            .char_indices()
            .find_map(|(index, character)| if character.is_ascii_digit() { None } else { Some(index) })
            .unwrap_or_else(|| slice.len());

    let digits = &slice[0..digit_end];
    let remainder = &slice[digit_end..];

    (remainder, digits)
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
}