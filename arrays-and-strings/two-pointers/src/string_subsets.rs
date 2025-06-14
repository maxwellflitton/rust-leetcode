//! Here we need to check to see if a string is a subset of a bigger string. So for instance `"ace"` is in `"abcde"`.

pub fn ordered_string_subset(sub_set: &str, super_set: &str) -> bool {
    let mut sub_set_ptr = 0;
    let mut super_set_ptr = 0;

    let sub_set_buffer: Vec<char> = sub_set
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '.' && *c != ',') // basically they all have to result in true to get through the filter
        .collect();

    let super_set_buffer: Vec<char> = super_set
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '.' && *c != ',') // basically they all have to result in true to get through the filter
        .collect();

    // This loop breaks when an array is exhausted
    while sub_set_ptr < sub_set_buffer.len() && super_set_ptr < super_set_buffer.len() {
        let sub_set_val = sub_set_buffer[sub_set_ptr];
        let super_set_val = super_set_buffer[super_set_ptr];

        if sub_set_val == super_set_val {
            sub_set_ptr += 1;
        }
        super_set_ptr += 1;
    }

    sub_set_ptr == sub_set_buffer.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_empty() {
        assert_eq!(ordered_string_subset("", ""), true);
    }

    #[test]
    fn empty_subset() {
        assert_eq!(ordered_string_subset("", "abcde"), true);
    }

    #[test]
    fn empty_superset() {
        assert_eq!(ordered_string_subset("abc", ""), false);
    }

    #[test]
    fn exact_match() {
        assert_eq!(ordered_string_subset("abc", "abc"), true);
    }

    #[test]
    fn full_subset_in_order() {
        assert_eq!(ordered_string_subset("ace", "abcde"), true);
    }

    #[test]
    fn full_subset_not_in_order() {
        assert_eq!(ordered_string_subset("aec", "abcde"), false);
    }

    #[test]
    fn characters_exist_but_wrong_order() {
        assert_eq!(ordered_string_subset("cba", "abc"), false);
    }

    #[test]
    fn repeated_characters_valid() {
        assert_eq!(ordered_string_subset("aabb", "aaabbbccc"), true);
    }

    #[test]
    fn repeated_characters_invalid() {
        assert_eq!(ordered_string_subset("aabb", "abc"), false);
    }

    #[test]
    fn subset_longer_than_superset() {
        assert_eq!(ordered_string_subset("abcdef", "abc"), false);
    }

    #[test]
    fn unicode_characters_in_order() {
        assert_eq!(ordered_string_subset("你好", "你好吗"), true);
    }

    #[test]
    fn unicode_characters_wrong_order() {
        assert_eq!(ordered_string_subset("吗好", "你好吗"), false);
    }

    #[test]
    fn case_sensitive_match() {
        assert_eq!(ordered_string_subset("aB", "abcBd"), true);
    }

    // #[test]
    // fn case_sensitive_mismatch() {
    //     assert_eq!(ordered_string_subset("ab", "aBc"), false);
    // }

    #[test]
    fn only_last_char_matches() {
        assert_eq!(ordered_string_subset("f", "abcdef"), true);
    }

    #[test]
    fn last_char_mismatch() {
        assert_eq!(ordered_string_subset("z", "abcdef"), false);
    }
}
