//! For an input such as the following:
//! ```
//! nums = [-3,2,-3,4,2]
//! ```
//! We have a start value that will produce a positive streak all the way.

pub fn min_start_value(nums: Vec<i32>) -> i32 {
    // placeholder needs to be the start because if this is negative
    // we still need a positive number
    let mut placeholder = nums[0];
    let mut left_ptr = 1;
    let mut buffer = vec![0; nums.len()];
    buffer[0] = nums[0];

    while left_ptr < nums.len() {
        buffer[left_ptr] = nums[left_ptr] + buffer[left_ptr - 1];
        if buffer[left_ptr] < placeholder {
            placeholder = buffer[left_ptr];
        }
        left_ptr += 1;
    }
    // if the minimum prefix is negative, we need 1 - min_prefix;
    // otherwise the smallest positive start value is just 1
    if placeholder < 0 { 1 - placeholder } else { 1 }
}

#[cfg(test)]
mod tests {
    use super::min_start_value;

    #[test]
    fn sample1() {
        assert_eq!(min_start_value(vec![-3, 2, -3, 4, 2]), 5);
    }
    #[test]
    fn sample2() {
        assert_eq!(min_start_value(vec![1, 2]), 1);
    }
    #[test]
    fn sample3() {
        assert_eq!(min_start_value(vec![1, -2, -3]), 5);
    }

    #[test]
    fn single_pos() {
        assert_eq!(min_start_value(vec![5]), 1);
    }
    #[test]
    fn single_neg() {
        assert_eq!(min_start_value(vec![-4]), 5);
    }

    #[test]
    fn hits_zero() {
        assert_eq!(min_start_value(vec![2, 3, -5, 4]), 1);
    }
    #[test]
    fn mid_valley() {
        assert_eq!(min_start_value(vec![-2, -3, 7]), 6);
    }
    #[test]
    fn deep_first() {
        assert_eq!(min_start_value(vec![-10, 5, 5]), 11);
    }

    #[test]
    fn all_zero() {
        assert_eq!(min_start_value(vec![0, 0, 0]), 1);
    }
    #[test]
    fn all_neg_small() {
        assert_eq!(min_start_value(vec![-1, -1, -1]), 4);
    }

    #[test]
    fn zigzag() {
        assert_eq!(min_start_value(vec![4, -5, 2, -1, 2]), 2);
    }
    #[test]
    fn mono_down() {
        assert_eq!(min_start_value(vec![-1, -2, -3, -4]), 11);
    }
    #[test]
    fn net_positive() {
        assert_eq!(min_start_value(vec![10, -12, 5]), 3);
    }
    #[test]
    fn shallow_final() {
        assert_eq!(min_start_value(vec![-5, 1, 1, 1, 1, 1]), 6);
    }
    #[test]
    fn alt_ones() {
        assert_eq!(min_start_value(vec![1, -1, 1, -1]), 1);
    }

    #[test]
    fn large_vals() {
        assert_eq!(min_start_value(vec![-100_000, 99_999, 99_999]), 100_001);
    }

    #[test]
    fn stress_long() {
        let big = vec![-1; 100_000];
        assert_eq!(min_start_value(big), 100_001);
    }
}
