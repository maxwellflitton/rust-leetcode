pub fn find_best_subarray(nums: &[i32], k: usize) -> i32 {
    assert!(k > 0 && k <= nums.len(), "`k` must be in 1..=len(nums)");

    // ── 1.  Build the initial window [0 .. k-1] ─────────────────────────────
    let mut left_ptr: usize = 0;
    let mut right_ptr: usize = 0;
    let mut current_sum: i32 = 0;

    // prefil the first length
    while right_ptr < k {
        current_sum += nums[right_ptr];
        right_ptr += 1;
    }
    // `right_ptr == k` now; window size is exactly k
    let mut best_sum = current_sum;

    // loop through the rest of the array as a fixed window
    while right_ptr < nums.len() {
        // add new element entering on the right
        current_sum += nums[right_ptr];

        // remove element exiting on the left
        current_sum -= nums[left_ptr];

        // advance both pointers
        left_ptr += 1;
        right_ptr += 1;

        // track the best (maximum) window sum seen so far
        if current_sum > best_sum {
            best_sum = current_sum;
        }
    }

    best_sum
}

#[cfg(test)]
mod tests {
    use super::find_best_subarray;

    // ───────────────────────────────
    // helpers
    // ───────────────────────────────
    macro_rules! case {
        ($nums:expr, $k:expr => $expect:expr) => {
            assert_eq!(find_best_subarray(&$nums, $k), $expect);
        };
    }

    // ───────────────────────────────
    // basic known examples
    // ───────────────────────────────
    #[test]
    fn textbook_example() {
        // window 3 -> [5,1,3] has max sum 9
        case!([2, 1, 5, 1, 3, 2], 3 => 9);
    }

    #[test]
    fn prompt_variant() {
        // first k window already max
        case!([1, 2, 3, 4, 5], 2 => 9);
    }

    // ───────────────────────────────
    // k edge values
    // ───────────────────────────────
    #[test]
    fn k_equals_one() {
        // should pick max element
        case!([-2, -8, 7, -1, 3], 1 => 7);
    }

    #[test]
    fn k_equals_len() {
        case!([3, 4, 5], 3 => 12);
    }

    #[test]
    fn k_equals_len_minus_one() {
        // best window drops the smallest element (-10)
        case!([-10, 1, 2, 3, 4], 4 => 10);
    }

    // ───────────────────────────────
    // all negative
    // ───────────────────────────────
    #[test]
    fn all_negatives() {
        // choose least-negative sum (-6)
        case!([-5, -4, -3, -2, -1], 3 => -6);
    }

    // ───────────────────────────────
    // mixed signs
    // ───────────────────────────────
    #[test]
    fn positives_and_negatives() {
        case!([5, -2, 6, 6, -1, 4], 3 => 11); // 11 is correct
    }

    #[test]
    fn zeros_inside() {
        case!([0, 0, 7, 0, 0], 3 => 7);
    }

    // ───────────────────────────────
    // duplicates / flat array
    // ───────────────────────────────
    #[test]
    fn constant_values() {
        case!([4, 4, 4, 4], 2 => 8);
    }

    // ───────────────────────────────
    // large-ish sanity
    // ───────────────────────────────
    #[test]
    fn ascending_one_to_hundred() {
        let nums: Vec<i32> = (1..=100).collect();
        // best window of size 10 is the last 10 numbers
        let expected: i32 = (91..=100).sum();
        case!(nums, 10 => expected);
    }

    // ───────────────────────────────
    // panic cases (k invalid)
    // ───────────────────────────────
    #[test]
    #[should_panic]
    fn k_zero_panics() {
        find_best_subarray(&[1, 2, 3], 0);
    }

    #[test]
    #[should_panic]
    fn k_too_large_panics() {
        find_best_subarray(&[1, 2, 3], 4);
    }
}
