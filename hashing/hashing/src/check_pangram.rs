use std::collections::HashSet;

/// Check is the string contains every letter in the alphabet.
pub fn check_if_pangram(sentence: String) -> bool {
    let mut char_set = HashSet::new();
    let mut count = 0;
    for i in sentence.chars() {
        match char_set.insert(i) {
            true => count += 1,
            false => {}
        }
        // early exit
        if count == 26 {
            return true;
        }
    }
    false
}
