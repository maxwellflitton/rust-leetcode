//! Here we need to calculate the number of sub arrays in an array that are less than a total value. This can be a lot
//! so we need the following steps:
//!
//! # Steps
//! 1. left and right ptr are zero
//! 2. current value has to be `1` because we're dealing with multiplication
//! 3. loop through the iterator with the right pointer
//! 4. current is multiplied by the read from the right pointer
//! 5. while loop to reduce the window size by dividing by the left read and increasing the left ptr

pub fn total_number_of_subarrays(nums: Vec<i32>, total_val: i32) -> usize {
    // process the inputs
    if total_val <= 1 {
        return 0;
    }

    // define the meta data
    let mut left_ptr = 0;
    let mut right_ptr = 0;
    let mut current_value = 1_i64; // avoid overflow
    let mut total_found = 0_usize;

    // loop through the entire iterator
    while right_ptr < nums.len() {
        // ① expand window to the right
        current_value *= nums[right_ptr] as i64;

        // ② shrink window from the left until product < total_val
        while current_value >= total_val as i64 {
            current_value /= nums[left_ptr] as i64;
            left_ptr += 1;
        }

        // ③ count sub-arrays ending at right_ptr
        if left_ptr <= right_ptr {
            // <-- guard added
            total_found += right_ptr - left_ptr + 1;
        }

        right_ptr += 1; // ④ move on
    }

    total_found
}

#[cfg(test)]
mod tests {
    use super::total_number_of_subarrays;

    // Small helper macro → less boiler-plate
    macro_rules! case {
        ( $nums:expr, $k:expr => $expect:expr ) => {
            assert_eq!(total_number_of_subarrays($nums.to_vec(), $k), $expect);
        };
    }

    // ——————————————————————————————————
    // 1. Prompt example
    // ——————————————————————————————————
    #[test]
    fn prompt_example() {
        case!(&[10, 5, 2, 6], 100 => 8);
    }

    // ——————————————————————————————————
    // 2. Trivial & empty
    // ——————————————————————————————————
    #[test]
    fn empty_array() {
        case!(&[], 50 => 0);
    }

    #[test]
    fn single_ok() {
        case!(&[7], 8 => 1); // product 7 < 8
    }

    #[test]
    fn single_equal_k() {
        case!(&[5], 5 => 0); // strictly less!
    }

    #[test]
    fn single_above_k() {
        case!(&[9], 3 => 0);
    }

    // ——————————————————————————————————
    // 3. k ≤ 1 edge (all nums are positive)
    // ——————————————————————————————————
    #[test]
    fn k_zero() {
        case!(&[1, 2, 3], 0 => 0);
    }

    #[test]
    fn k_one() {
        case!(&[1, 1, 1], 1 => 0); // need strictly < 1
    }

    // ——————————————————————————————————
    // 4. Runs of ones (many valid windows)
    // n ones → n*(n+1)/2 subarrays
    // ——————————————————————————————————
    #[test]
    fn five_ones_k_2() {
        case!(&[1, 1, 1, 1, 1], 2 => 15); // 5*6/2
    }

    #[test]
    fn three_ones_k_large() {
        case!(&[1, 1, 1], 100 => 6); // full power set of contiguous slices
    }

    // ——————————————————————————————————
    // 5. Mixed small array
    // ——————————————————————————————————
    #[test]
    fn mixed_small() {
        // Subarrays < 6: [1], [2], [3], [1,2]  → total 4
        case!(&[1, 2, 3], 6 => 4);
    }

    // ——————————————————————————————————
    // 6. No valid subarrays
    // ——————————————————————————————————
    #[test]
    fn none_fit() {
        case!(&[100, 200], 100 => 0);
    }

    // ——————————————————————————————————
    // 7. Longer sanity check
    // ——————————————————————————————————
    #[test]
    fn long_progression() {
        // nums = 1..=10  (all factorial-growing products)
        // Choose k so only singletons are allowed
        let nums: Vec<i32> = (1..=10).collect();
        assert_eq!(total_number_of_subarrays(nums.clone(), 11), 13);
    }
}
