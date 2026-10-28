// Given a string s, find the length of the longest substring without duplicate characters.

// Example 1:

// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3. Note that "bca" and "cab" are also correct answers.
// Example 2:

// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.
// Example 3:

// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.

// Constraints:

// 0 <= s.length <= 5 * 104
// s consists of English letters, digits, symbols and spaces.
use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();

    let mut seen = HashSet::new();
    let mut longest = 0;
    let mut l_ptr = 0;

    for r_ptr in 0..chars.len() {
        let current_char = chars[r_ptr];

        // roll up untill there isn't a duplicate
        while seen.contains(&current_char) {
            let remove_char = chars[l_ptr];
            seen.remove(&remove_char);
            l_ptr += 1;
        }

        // add to the new one
        seen.insert(current_char);
        let delta = r_ptr - l_ptr + 1;
        if delta > longest {
            longest = delta;
        }
    }
    longest as i32
}
