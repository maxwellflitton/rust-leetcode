//! Here is sort squared numbers. so `[-4,-1,0,3,10]` needs to be sorted to `[16,1,0,9,100]`
//!
//! # To Solve
//! 1. Here we have a left pointer and a right pointer at both ends
//! 2. We also have a third pointer, a writer pointer that starts at the right (highest number)
//! 3. At each iteration the writer pointer moves left because we're writing at each iteration
//! 3. What gets written is the largest, either the left or right pointer.
//! 4. Left or right pointer gets moved depending on which one is written
//!
//! # Note
//! This only works because inputs are still kind of sorted from smallest to largest. We need the
//! third writer pointer because the negative numbers become positive but it's not an overall sorting algorithm
//!
//! # How to identify the problem here
//! 1. Notice the property about squares being in order for sorted positives, backward for sorted negatives.
//! 2. Think of strategies, such as using two pointers, similar to how you would merge two sorted arrays.
//! 3. Utilize that idea in this context, essentially merging a positive and a negative array.
//!
//! # Hint
//! Leetcode might not be super relevant to actual work, but the skills of being able to identify,
//! adapt, and apply similar solutions to new problems may be useful. When you're studying,
//! don't just read a solution and pass the problem, but really take time to take note of the
//! techniques used, and why they're useful to solving that specific problem.

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    // setup the pointers
    let mut left_ptr = 0;
    let mut right_ptr = match nums.len() {
        0 => 0,
        _ => nums.len() - 1,
    };
    let mut write_idx = right_ptr;

    // setup the buffer to contain the solution
    let mut buffer = vec![0; nums.len()];

    // handle edgecases
    if nums.is_empty() {
        return nums;
    } else if nums.len() == 1 {
        buffer[0] = nums[0] * nums[0];
        return buffer;
    }

    // This loop prevents the pointers from passing but there's a risk of underflowing
    while left_ptr <= right_ptr {
        // calculate the values
        let left_sq = nums[left_ptr] * nums[left_ptr];
        let right_sq = nums[right_ptr] * nums[right_ptr];

        // write the biggest value to the writer pointer
        if left_sq >= right_sq {
            buffer[write_idx] = left_sq;
            left_ptr += 1;
        } else {
            buffer[write_idx] = right_sq;
            // right is usize, so use `saturating_sub`
            right_ptr = right_ptr.saturating_sub(1);
        }

        // break to prevent underflow
        if write_idx == 0 {
            break;
        }
        // the value has been written so we always
        write_idx -= 1;
    }

    buffer
}

#[cfg(test)]
mod tests {
    use super::sorted_squares;

    // ---------- helpers ----------
    /// Convenience macro to avoid typing `vec![ … ]` on both sides.
    macro_rules! case {
        ( [$($input:expr),*] => [$($expected:expr),*] $(,)? ) => {
            assert_eq!(
                sorted_squares(vec![$($input),*]),
                vec![$($expected),*]
            );
        };
    }

    // ---------- basic shape ----------
    #[test]
    fn single_zero() {
        case!([0] => [0]);
    }

    #[test]
    fn single_positive() {
        case!([7] => [49]);
    }

    #[test]
    fn single_negative() {
        case!([-3] => [9]);
    }

    // ---------- homogeneous sign ----------
    #[test]
    fn all_positive_sorted() {
        case!([1, 2, 3, 4] => [1, 4, 9, 16]);
    }

    #[test]
    fn all_negative_sorted() {
        // Input is non-decreasing, so most-negative is first
        case!([-9, -7, -2] => [4, 49, 81]);
    }

    // ---------- zeros & duplicates ----------
    #[test]
    fn zeros_and_duplicates() {
        case!([-2, -2, 0, 0, 3] => [0, 0, 4, 4, 9]);
    }

    // ---------- mixed signs ----------
    #[test]
    fn mixed_balanced() {
        case!([-4, -1, 0, 3, 10] => [0, 1, 9, 16, 100]);
    }

    #[test]
    fn mixed_unbalanced_more_negatives() {
        case!([-7, -3, -1, 2] => [1, 4, 9, 49]);
    }

    #[test]
    fn mixed_unbalanced_more_positives() {
        case!([-1, 0, 1, 2, 3, 4] => [0, 1, 1, 4, 9, 16]);
    }

    // ---------- extreme values within constraint ----------
    #[test]
    fn extremes_at_bounds() {
        case!([-10_000, 0, 10_000] => [0, 100_000_000, 100_000_000]);
    }

    // ---------- already-squared (algorithm must still sort) ----------
    #[test]
    fn input_already_non_decreasing_but_not_original_order() {
        // 4 and 9 appear before 1 in the squared sense
        case!([-9, -2, -1] => [1, 4, 81]);
    }

    // ---------- long vector sanity ----------
    #[test]
    fn long_sequence_odd_length() {
        let mut input: Vec<i32> = (-50..=49).collect(); // 100 numbers
        let mut expected: Vec<i32> = input.iter().map(|n| n * n).collect();
        expected.sort(); // squares must be sorted
        assert_eq!(sorted_squares(input), expected);
    }
}
