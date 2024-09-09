use crate::is_even::IsEven;
use regex::Regex;

pub fn has_multiple_words_between_braces(s: &str) -> bool {
    let re = Regex::new(r"\{\{?\s*([^}]+)\s*\}?\}").unwrap();

    if let Some(captures) = re.captures(s) {
        let content = &captures[1].trim();
        let words: Vec<&str> = content.split_whitespace().collect();
        return words.len() > 1;
    }

    false
}

pub fn count_left_braces(s: &str) -> usize {
    s.matches("{").count()
}

pub fn count_right_braces(s: &str) -> usize {
    s.matches("}").count()
}

pub fn has_even_left_braces(s: &str) -> bool {
    count_left_braces(s).is_even()
}

pub fn has_even_right_braces(s: &str) -> bool {
    count_right_braces(s).is_even()
}

pub fn has_left_brace(s: &str) -> bool {
    count_left_braces(s) > 0
}

pub fn has_right_brace(s: &str) -> bool {
    count_right_braces(s) > 0
}

pub fn has_consecutive_left_braces(s: &str) -> bool {
    s.contains("{{")
}

pub fn has_consecutive_right_braces(s: &str) -> bool {
    s.contains("}}")
}

pub fn has_only_single_braces(s: &str) -> bool {
    has_left_brace(s)
        && has_right_brace(s)
        && !has_consecutive_left_braces(s)
        && !has_consecutive_right_braces(s)
}

pub fn has_only_double_braces(s: &str) -> bool {
    has_consecutive_left_braces(s)
        && has_consecutive_right_braces(s)
        && has_even_left_braces(s)
        && has_even_right_braces(s)
}

pub fn has_no_braces(s: &str) -> bool {
    !has_left_brace(s) && !has_right_brace(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_multiple_words_between_braces() {
        assert!(has_multiple_words_between_braces("{one two}"));
        assert!(has_multiple_words_between_braces("{ one two three }"));
        assert!(has_multiple_words_between_braces(
            "{one two} and {three four}"
        ));

        assert!(!has_multiple_words_between_braces("{ one }"));

        assert!(has_multiple_words_between_braces("{{one two}}"));
        assert!(has_multiple_words_between_braces("{{ one two three }}"));
        assert!(has_multiple_words_between_braces(
            "{{one two}} and {{three four}}"
        ));

        assert!(!has_multiple_words_between_braces("{{ one }}"));
    }

    #[test]
    fn test_count_left_braces() {
        assert_eq!(count_left_braces("hello {world"), 1);
        assert_eq!(count_left_braces("hello world}"), 0);
        assert_eq!(count_left_braces("hello"), 0);
        assert_eq!(count_left_braces("hello {big} {{world}}"), 3);
        assert_eq!(count_left_braces("hello {{world}}"), 2);
    }

    #[test]
    fn test_count_right_braces() {
        assert_eq!(count_right_braces("hello {world"), 0);
        assert_eq!(count_right_braces("hello world}"), 1);
        assert_eq!(count_right_braces("hello"), 0);
        assert_eq!(count_right_braces("hello {big} {{world}}"), 3);
        assert_eq!(count_right_braces("hello {{world}}"), 2);
    }

    #[test]
    fn test_has_even_left_braces() {
        assert!(has_even_left_braces("hello {world} {world}"));
        assert!(has_even_left_braces("hello world}"));
        assert!(!has_even_left_braces("hello {world"));
        assert!(!has_even_left_braces("hello {world}"));
        assert!(!has_even_left_braces("hello {world} {world} {world}"));
        assert!(!has_even_left_braces("hello {world} {{world}}"));
    }

    #[test]
    fn test_has_even_right_braces() {
        assert!(has_even_right_braces("hello {world} {world}"));
        assert!(has_even_right_braces("hello {world"));
        assert!(!has_even_right_braces("hello {world}"));
        assert!(!has_even_right_braces("hello {world} {world} {world}"));
        assert!(!has_even_right_braces("hello {world} {{world}}"));
    }

    #[test]
    fn test_has_left_brace() {
        assert!(has_left_brace("hello {world}"));
        assert!(has_left_brace("hello {world"));

        assert!(!has_left_brace("hello"));
        assert!(!has_left_brace("hello world}"));
    }

    #[test]
    fn test_has_right_brace() {
        assert!(has_right_brace("hello world}"));
        assert!(has_right_brace("hello {world}"));

        assert!(!has_right_brace("hello"));
        assert!(!has_right_brace("hello {world"));
    }

    #[test]
    fn test_has_consecutive_left_braces() {
        assert!(has_consecutive_left_braces("hello {{world}"));
        assert!(has_consecutive_left_braces("hello {{world"));

        assert!(!has_consecutive_left_braces("hello"));
        assert!(!has_consecutive_left_braces("hello {world}"));
    }

    #[test]
    fn test_has_consecutive_right_braces() {
        assert!(has_consecutive_right_braces("hello world}}"));
        assert!(has_consecutive_right_braces("hello {world}}"));

        assert!(!has_consecutive_right_braces("hello"));
        assert!(!has_consecutive_right_braces("hello {world}"));
    }

    #[test]
    fn test_has_single_braces() {
        assert!(has_only_single_braces("hello {world}"));
        assert!(has_only_single_braces("hello {world} {world}"));

        assert!(!has_only_single_braces("hello {world"));
        assert!(!has_only_single_braces("hello"));
        assert!(!has_only_single_braces("hello world}"));
        assert!(!has_only_single_braces("hello {{world}} {world}"));
        assert!(!has_only_single_braces("hello {world} {{world}}"));
    }

    #[test]
    fn test_has_double_braces() {
        assert!(has_only_double_braces("hello {{world}}"));
        assert!(has_only_double_braces("hello {{world}} {{world}}"));

        assert!(!has_only_double_braces("hello {{world"));
        assert!(!has_only_double_braces("hello"));
        assert!(!has_only_double_braces("hello world}}"));
        assert!(!has_only_double_braces("hello {world} {{world}}"));
        assert!(!has_only_double_braces("hello {{world}} {world}"));
    }

    #[test]
    fn test_has_no_braces() {
        assert!(has_no_braces("hello world"));
        assert!(has_no_braces("hello"));

        assert!(!has_no_braces("hello {world}"));
        assert!(!has_no_braces("hello {world} {world}"));
        assert!(!has_no_braces("hello {{world}}"));
        assert!(!has_no_braces("hello {{world}} {{world}}"));
    }
}
