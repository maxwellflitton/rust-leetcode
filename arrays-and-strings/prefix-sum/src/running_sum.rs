pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut ptr = 1;
    let mut buffer = vec![0; nums.len()];
    buffer[0] = nums[0];
    while ptr < nums.len() {
        buffer[ptr] = nums[ptr] + buffer[ptr - 1];
        ptr += 1;
    }
    buffer
}

#[cfg(test)]
mod tests {
    use super::running_sum;

    // ────────────────
    // Examples
    // ────────────────
    #[test]
    fn example_1() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![1, 3, 6, 10];
        assert_eq!(running_sum(nums), expected);
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 1, 1, 1, 1];
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(running_sum(nums), expected);
    }

    #[test]
    fn example_3() {
        let nums = vec![3, 1, 2, 10, 1];
        let expected = vec![3, 4, 6, 16, 17];
        assert_eq!(running_sum(nums), expected);
    }

    // ────────────────
    // Tiny / edge inputs
    // ────────────────
    #[test]
    fn single_element() {
        assert_eq!(running_sum(vec![42]), vec![42]);
    }

    #[test]
    fn all_zeros() {
        assert_eq!(running_sum(vec![0, 0, 0]), vec![0, 0, 0]);
    }

    #[test]
    fn negatives_and_positives() {
        let nums = vec![5, -3, 2, -4, 6];
        let expected = vec![5, 2, 4, 0, 6];
        assert_eq!(running_sum(nums), expected);
    }

    // ────────────────
    // Stress / large input
    // ────────────────
    #[test]
    fn large_input() {
        // Build Vec<[1; 100_000]> at run-time
        let n = 100_000;
        let nums = vec![1i32; n];
        let result = running_sum(nums.clone());

        // The last element of the running sum of n ones should be n
        assert_eq!(result.len(), n);
        assert_eq!(result[n - 1], n as i32);

        // Spot-check a few positions
        assert_eq!(result[0], 1);
        assert_eq!(result[9_999], 10_000);
        assert_eq!(result[50_000], 50_001);

        // Optional: verify whole output efficiently
        for (idx, val) in result.iter().enumerate() {
            assert_eq!(*val, (idx as i32) + 1);
        }
    }
}
