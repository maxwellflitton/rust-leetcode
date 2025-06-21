pub fn min_start_value(nums: Vec<i32>) -> i32 {
    let mut positive_streak = false;
    let mut placeholder = 0;
    let mut left_ptr = 1;
    let mut buffer = vec![0; nums.len()];
    buffer[0] = nums[0];

    if buffer[0] > 0 {
        positive_streak = true;
    }

    while left_ptr < nums.len() {
        buffer[left_ptr] = nums[left_ptr] + buffer[left_ptr - 1];
        // flip to placeholder
        if buffer[left_ptr] > 0 && !positive_streak {
            placeholder = left_ptr;
            positive_streak = true;
        } else if buffer[left_ptr] < 0 && positive_streak {
            positive_streak = false;
        }
        println!("{:?}", buffer);
        left_ptr += 1;
    }
    (placeholder + 1) as i32
}

#[cfg(test)]
mod tests {
    use super::min_start_value;

    #[test]
    fn test_one() {
        let nums = vec![-3, 2, -3, 4, 2];
        println!("{:?}", vec![-3, 2, -3, 4, 2]);
        let outcome = min_start_value(nums);
        println!("{}", outcome);
    }

    /// Official example from the problem statement
    #[test]
    fn example_case() {
        let nums = vec![-3, 2, -3, 4, 2];
        assert_eq!(min_start_value(nums), 5);
    }

    /// Sequence where the running sum never drops below 1 → answer should be 1
    #[test]
    fn all_positive() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(min_start_value(nums), 1);
    }

    /// All negative numbers: we need 1 − (sum of all) as the start value
    #[test]
    fn all_negative() {
        let nums = vec![-2, -3, -4];
        // prefix minima: -2, -5, -9  → start = 1 - (-9) = 10
        assert_eq!(min_start_value(nums), 10);
    }

    // /// A single negative element
    // #[test]
    // fn single_negative() {
    //     let nums = vec![-1];
    //     assert_eq!(min_start_value(nums), 2);
    // }

    // /// A single positive element
    // #[test]
    // fn single_positive() {
    //     let nums = vec![7];
    //     assert_eq!(min_start_value(nums), 1);
    // }

    // /// Mixed ups and downs
    // #[test]
    // fn mixed_sequence() {
    //     let nums = vec![-5, 4, 1, 7, 2, -6, 5];
    //     // prefix minima: -5, -1, 0, 7, 9, 3, 8 → min = -5 → start = 6
    //     assert_eq!(min_start_value(nums), 6);
    // }

    // /// Zeros shouldn’t change the need for the minimal positive start value
    // #[test]
    // fn zeros_only() {
    //     let nums = vec![0, 0, 0, 0];
    //     assert_eq!(min_start_value(nums), 1);
    // }

    // /// If an empty vector is considered valid input, answer should be 1
    // #[test]
    // fn empty_vector() {
    //     let nums: Vec<i32> = vec![];
    //     assert_eq!(min_start_value(nums), 1);
    // }

    // /// Large but still safe values (makes sure we don’t over- or underflow)
    // #[test]
    // fn large_values() {
    //     let nums = vec![1_000_000_000, -1_000_000_000];
    //     assert_eq!(min_start_value(nums), 1);
    // }
}
