//! Here is where we can work out if the string can be reversed like "hannah". We can this with two pointers.
//! This approach for looping is good for blindly traversing or reversing something.

pub fn assert_palidrome(input_str: &str) -> bool {
    // this is better than just the length because the str length is on byte length and we iterate over chars. With
    // newer chars they can be more than one byte resulting in indexes out of range
    let clean_str: Vec<char> = input_str
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '.' && *c != ',') // basically they all have to result in true to get through the filter
        .collect();
    let mut left_ptr = 0;
    let mut right_ptr = clean_str.len().saturating_sub(1); // avoids underflow on empty strings

    // if the left pointer is the same then this triggers
    while left_ptr < right_ptr {
        if clean_str[left_ptr] != clean_str[right_ptr] {
            return false;
        }
        left_ptr += 1;
        right_ptr -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::assert_palidrome;

    #[test]
    fn test_palindromes() {
        let cases = vec![
            ("", true),  // empty string
            ("a", true), // single char
            ("aa", true),
            ("aba", true),
            ("abba", true),
            ("racecar", true),
            ("RaceCar", true),                     // case insensitivity?
            ("A man a plan a canal Panama", true), // common sentence palindrome
            ("No lemon, no melon", true),          // with punctuation and case
            // Unicode palindromes
            ("あいいあ", true), // Japanese
            ("😺😸😺", true),   // emoji
        ];

        for (s, expected) in cases {
            let result = assert_palidrome(s);
            assert_eq!(
                result, expected,
                "Failed on input: '{}'. Expected {}, got {}",
                s, expected, result
            );
        }
    }

    #[test]
    fn test_non_palindromes() {
        let cases = vec![
            ("hello", false),
            ("abc", false),
            ("ab", false),
            ("palindrome", false),
            ("Not a Palindrome", false),
            ("😀😃😄", false), // different emoji
        ];

        for (s, expected) in cases {
            let result = assert_palidrome(s);
            assert_eq!(
                result, expected,
                "Failed on input: '{}'. Expected {}, got {}",
                s, expected, result
            );
        }
    }
}
