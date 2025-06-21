//! Given a binary array nums and an integer k, return the maximum number of consecutive 1's in the array if you can flip at most k 0's.

pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut left_ptr = 0;
    let mut right_ptr = 0;

    let mut top_len = 0;
    let mut zero_count = 0;

    while right_ptr < nums.len() {
        // read the new advance
        let read = nums[right_ptr];
        if read == 0 {
            zero_count += 1;
        }

        // move the left pointer till params are satisfied
        // (remember don't try anything fancy like trying to check if the pointers are equal)
        while zero_count > k {
            if nums[left_ptr] == 0 {
                zero_count -= 1;
            }
            left_ptr += 1;
        }

        // update the meta data
        let cached_len = right_ptr - left_ptr + 1;
        if cached_len > top_len {
            top_len = cached_len;
        }

        // increase the pointer
        right_ptr += 1;
    }

    top_len as i32
}

#[cfg(test)]
mod tests {
    use super::longest_ones;

    // ────────────────────────────────
    // Basic examples from the problem
    // ────────────────────────────────
    #[test]
    fn example_1() {
        let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
        assert_eq!(longest_ones(nums, 2), 6);
    }

    #[test]
    fn example_2() {
        let nums = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
        assert_eq!(longest_ones(nums, 3), 10);
    }

    // ────────────────────────────────
    // Tiny inputs
    // ────────────────────────────────
    #[test]
    fn single_one_no_flips() {
        assert_eq!(longest_ones(vec![1], 0), 1);
    }

    #[test]
    fn single_zero_with_flip() {
        assert_eq!(longest_ones(vec![0], 1), 1);
    }

    // ────────────────────────────────
    // All-ones / all-zeros
    // ────────────────────────────────
    #[test]
    fn all_ones_no_flips_needed() {
        let nums = vec![1, 1, 1, 1, 1, 1, 1];
        assert_eq!(longest_ones(nums, 0), 7);
    }

    #[test]
    fn all_zeros_no_flips() {
        let nums = vec![0, 0, 0, 0];
        assert_eq!(longest_ones(nums, 0), 0);
    }

    #[test]
    fn all_zeros_enough_flips() {
        let nums = vec![0, 0, 0, 0];
        assert_eq!(longest_ones(nums.clone(), nums.len() as i32), 4);
    }

    // ────────────────────────────────
    // Alternating patterns
    // ────────────────────────────────
    #[test]
    fn alternating_start_zero_two_flips() {
        let nums = vec![0, 1, 0, 1, 0, 1, 0, 1];
        assert_eq!(longest_ones(nums, 2), 5); // best window: 1 0 1 0 1 → 1 1 1 1 1
    }

    #[test]
    fn alternating_start_one_one_flip() {
        let nums = vec![1, 0, 1, 0, 1];
        assert_eq!(longest_ones(nums, 1), 3);
    }

    // ────────────────────────────────
    // Zeros at the edges
    // ────────────────────────────────
    #[test]
    fn zeros_head_tail_two_flips() {
        let nums = vec![0, 0, 1, 1, 1, 0, 0];
        assert_eq!(longest_ones(nums, 2), 5);
    }

    // ────────────────────────────────
    // k larger than the number of zeros
    // ────────────────────────────────
    #[test]
    fn flips_exceed_zeros() {
        let nums = vec![1, 0, 1];
        assert_eq!(longest_ones(nums, 5), 3);
    }

    // ────────────────────────────────
    // Stress / upper-bound size cases
    // ────────────────────────────────
    #[test]
    fn large_all_ones() {
        let n = 100_000;
        let nums = vec![1i32; n];
        assert_eq!(longest_ones(nums, 0), n as i32);
    }

    #[test]
    fn large_half_ones_half_zeros_big_k() {
        let n = 50_000;
        let mut nums = vec![1i32; n];
        nums.extend(vec![0i32; n]); // now len = 100 000
        assert_eq!(longest_ones(nums, n as i32), 100_000);
    }
}
