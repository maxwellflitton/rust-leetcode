//! Here is where we need to reverse a string.
//!
//! # Steps I'd take
//! 1. setup the pointers
//! 2. setup the loop (this loop just traverses the entire char set so while left_ptr < right_ptr)
//! 3. define the increase pointer logic so the loop isn't infinite
//! 4. define the mutation logic for the problem

pub fn reverse_string(s: &mut Vec<char>) {
    // define the pointers
    let mut left_ptr = 0;
    let mut right_ptr = match s.len() {
        0 => 0,
        _ => s.len() - 1,
    };

    // blindly loop through until the pointers cross
    while left_ptr < right_ptr {
        // split into two mutable slices for the right pointer
        let (left, right) = s.split_at_mut(right_ptr);
        // right[0] is always going to be zero otherwise it will be out of bounds
        std::mem::swap(&mut left[left_ptr], &mut right[0]);
        left_ptr += 1;
        right_ptr -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vec() {
        let mut s: Vec<char> = vec![];
        reverse_string(&mut s);
        assert_eq!(s, vec![]);
    }

    #[test]
    fn single_char() {
        let mut s = vec!['a'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['a']);
    }

    #[test]
    fn two_chars() {
        let mut s = vec!['a', 'b'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['b', 'a']);
    }

    #[test]
    fn odd_length() {
        let mut s = vec!['a', 'b', 'c'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['c', 'b', 'a']);
    }

    #[test]
    fn even_length() {
        let mut s = vec!['r', 'u', 's', 't'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['t', 's', 'u', 'r']);
    }

    #[test]
    fn with_spaces() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd'];
        reverse_string(&mut s);
        assert_eq!(
            s,
            vec!['d', 'l', 'r', 'o', 'w', ' ', 'o', 'l', 'l', 'e', 'h']
        );
    }

    #[test]
    fn with_symbols() {
        let mut s = vec!['!', '@', '#', '$'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['$', '#', '@', '!']);
    }

    #[test]
    fn with_unicode() {
        let mut s = vec!['你', '好', '👋'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['👋', '好', '你']);
    }

    #[test]
    fn palindromes() {
        let mut s = vec!['m', 'a', 'd', 'a', 'm'];
        let original = s.clone();
        reverse_string(&mut s);
        assert_eq!(s, original); // unchanged
    }
}
