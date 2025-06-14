//! Here we need to find two integers that some to a value in a sorted array.
//! This loop approach is good for targeting an outcome and moving the pointers
//! depending on the outcome and target.

pub fn two_sum(nums: Vec<i32>, target: i32) -> bool {
    let mut left_ptr = 0;
    let mut right_ptr = nums.len().saturating_sub(1);

    while left_ptr < right_ptr {
        let current = nums[left_ptr] + nums[right_ptr];
        if current == target {
            return true;
        }
        // if the current is larger than the target there's no point in increasing the left
        // because it will just make the next value bigger
        if current > target {
            right_ptr -= 1;
        } else {
            // if the current is smaller than the target we can go bigger on the left because
            // decreasing the right will only make it smaller
            left_ptr += 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn test_sorted_two_sum_found() {
        let cases = vec![
            (vec![1, 2, 3, 4, 6], 6, true),     // 2 + 4
            (vec![1, 2, 3, 4, 6], 10, true),    // 4 + 6
            (vec![-3, -1, 0, 2, 4], 1, true),   // -1 + 2
            (vec![0, 1, 2, 3, 5], 3, true),     // 0 + 3
            (vec![-5, -2, 1, 3, 7], 5, true),   // -2 + 7
            (vec![-10, -5, 0, 5, 10], 0, true), // -10 + 10 or -5 + 5
        ];

        for (nums, target, expected) in cases {
            let result = two_sum(nums.clone(), target);
            assert_eq!(
                result, expected,
                "Failed on sorted input {:?} with target {}",
                nums, target
            );
        }
    }

    #[test]
    fn test_sorted_two_sum_not_found() {
        let cases = vec![
            (vec![1, 2, 3, 4, 6], 13, false),
            (vec![-10, -5, -1], 1, false),
            (vec![1], 2, false), // single element
            (vec![], 0, false),  // empty list
            (vec![1, 2, 3], 100, false),
        ];

        for (nums, target, expected) in cases {
            let result = two_sum(nums.clone(), target);
            assert_eq!(
                result, expected,
                "Failed on sorted input {:?} with target {}",
                nums, target
            );
        }
    }
}
