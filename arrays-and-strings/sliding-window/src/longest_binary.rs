//! Here we need to find the longest number of 1s however, we are allowed 1 zero in the mix.

pub fn longest_binary(s: &str) -> usize {
    // clean the inputs
    let buffer: Vec<char> = s.chars().collect();

    // define the meta data that we are keeping track of
    let mut left_ptr = 0;
    let mut right_ptr = 0;
    let mut zeros_in_window = 0;
    let mut highest_len = 0;

    // loop through until the right pointer is at the end
    while right_ptr < buffer.len() {

        // add zero to meta data if it's true
        if buffer[right_ptr] == '0' {
            zeros_in_window += 1;
        }

        // shrink the window until it has ≤ 1 zero
        while zeros_in_window > 1 {
            if buffer[left_ptr] == '0' {
                zeros_in_window -= 1;
            }
            left_ptr += 1;
        }

        // window [left_ptr, right_ptr] is now valid
        let current_len = right_ptr - left_ptr + 1;
        highest_len = highest_len.max(current_len);

        right_ptr += 1; // expand to the right
    }

    highest_len
}

#[cfg(test)]
mod tests {
    use super::longest_binary;

    // ──────────────────────────────
    // Trivial inputs
    // ──────────────────────────────
    #[test]
    fn empty_string() {
        assert_eq!(longest_binary(""), 0);
    }

    #[test]
    fn single_zero() {
        assert_eq!(longest_binary("0"), 1); // flip it
    }

    #[test]
    fn single_one() {
        assert_eq!(longest_binary("1"), 1); // nothing to flip
    }

    // ──────────────────────────────
    // Two-character strings
    // ──────────────────────────────
    #[test]
    fn double_zero() {
        assert_eq!(longest_binary("00"), 1); // flip either index
    }

    #[test]
    fn double_one() {
        assert_eq!(longest_binary("11"), 2);
    }

    #[test]
    fn one_zero_left() {
        assert_eq!(longest_binary("01"), 2); // flip index 0
    }

    #[test]
    fn one_zero_right() {
        assert_eq!(longest_binary("10"), 2); // flip index 1
    }

    // ──────────────────────────────
    // Short, mixed patterns
    // ──────────────────────────────
    #[test]
    fn alternating_mid_zero() {
        assert_eq!(longest_binary("101"), 3); // flip middle -> 111
    }

    #[test]
    fn edge_zero_connects_runs() {
        assert_eq!(longest_binary("1110"), 4); // flip last char
    }

    #[test]
    fn isolated_ones_far_apart() {
        // 1-0000-1  → longest is 2 no matter which zero flips
        assert_eq!(longest_binary("100001"), 2);
    }

    // ──────────────────────────────
    // Prompt example & variants
    // ──────────────────────────────
    #[test]
    fn example_prompt() {
        assert_eq!(longest_binary("1101100111"), 5);
    }

    #[test]
    fn two_zeros_inside_and_tail() {
        // 1110 11110 : flip inner zero →  eight consecutive 1s
        assert_eq!(longest_binary("111011110"), 8);
    }

    // ──────────────────────────────
    // All zeros / all ones
    // ──────────────────────────────
    #[test]
    fn all_zeros() {
        assert_eq!(longest_binary("00000"), 1);
    }

    #[test]
    fn all_ones() {
        assert_eq!(longest_binary("111111"), 6);
    }

    // ──────────────────────────────
    // Stress / large input
    // ──────────────────────────────
    #[test]
    fn large_balanced_runs() {
        // 500 × ‘1’, one ‘0’, 499 × ‘1’  → best = 1000
        let s = "1".repeat(500) + "0" + &"1".repeat(499);
        assert_eq!(longest_binary(&s), 1000);
    }
}
