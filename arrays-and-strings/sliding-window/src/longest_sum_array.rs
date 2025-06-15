//! Here we need to find the length of the longest array that sums to the total of a value. For instance:
//!
//! ```
//! k = 8
//! input = [3, 1, 2, 7, 4, 2, 1, 1, 5]
//! ```
//! The longest array that sums to `8` or under is `4`.

pub fn longest_array_for_sum(sum: i32, array: Vec<i32>) -> usize {
    let mut left_ptr = 0;
    let mut right_ptr = 0;

    // define the meta data that we're tracking
    let mut total_value = 0;
    let mut highest_len = 0;

    // ensure that the pointer is not out of bounds
    while right_ptr < array.len() {
        // always advance the right pointer at every iteration
        total_value += array[right_ptr];
        right_ptr += 1; // expand window

        // repeatedly shrink the window until the value is equal to of below the sum required
        while total_value > sum {
            total_value -= array[left_ptr];
            left_ptr += 1; // shrink window
        }

        // max is used because if a single value is higher than the sum the left poiinter
        // will surpass the right pointer when sheinking the window
        highest_len = highest_len.max(right_ptr - left_ptr);
    }

    highest_len
}

#[cfg(test)]
mod tests {
    use super::longest_array_for_sum;

    // ────────────────────────────────
    // Basic examples from the prompt
    // ────────────────────────────────
    #[test]
    fn example_from_description() {
        let k = 8;
        let input = vec![3, 1, 2, 7, 4, 2, 1, 1, 5];
        assert_eq!(longest_array_for_sum(k, input), 4); // [4,2,1,1]
    }

    // ────────────────────────────────
    // Trivial & edge-length cases
    // ────────────────────────────────
    #[test]
    fn empty_array() {
        assert_eq!(longest_array_for_sum(10, vec![]), 0);
    }

    #[test]
    fn single_element_equal_to_k() {
        assert_eq!(longest_array_for_sum(5, vec![5]), 1);
    }

    #[test]
    fn single_element_exceeds_k() {
        assert_eq!(longest_array_for_sum(3, vec![4]), 0);
    }

    // ────────────────────────────────
    // Whole array fits vs none fit
    // ────────────────────────────────
    #[test]
    fn entire_array_fits() {
        let v = vec![2, 3, 5];
        assert_eq!(longest_array_for_sum(10, v), 3);
    }

    #[test]
    fn no_subarray_fits() {
        let v = vec![6, 7, 8];
        assert_eq!(longest_array_for_sum(5, v), 0);
    }

    // ────────────────────────────────
    // Mixed sizes & duplicates
    // ────────────────────────────────
    #[test]
    fn multiple_candidates_choose_longest() {
        // k = 5 → longest is length-3 subarray [2,1,2]
        let v = vec![2, 1, 2, 1, 2];
        assert_eq!(longest_array_for_sum(5, v), 3);
    }

    #[test]
    fn k_smaller_than_smallest_element() {
        let v = vec![3, 4, 5];
        assert_eq!(longest_array_for_sum(2, v), 0);
    }

    // ────────────────────────────────
    // Special case: k == 0
    // ────────────────────────────────
    #[test]
    fn k_zero_all_zeros() {
        let v = vec![0, 0, 0];
        assert_eq!(longest_array_for_sum(0, v), 3);
    }

    #[test]
    fn k_zero_no_zeros() {
        let v = vec![1, 2, 3];
        assert_eq!(longest_array_for_sum(0, v), 0);
    }

    // ────────────────────────────────
    // Bigger input for sanity
    // ────────────────────────────────
    #[test]
    fn long_sequence_sanity_check() {
        // Generate 1..=20.  For k = 15, the best window is [1,2,3,4,5] → len 5
        let v: Vec<i32> = (1..=20).collect();
        assert_eq!(longest_array_for_sum(15, v), 5);
    }
}
