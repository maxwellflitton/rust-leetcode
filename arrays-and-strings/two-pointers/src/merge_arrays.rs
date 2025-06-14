//! Here is where we merge two sorted lists.

pub fn merge_arrays(one: Vec<i32>, two: Vec<i32>) -> Vec<i32> {
    let mut one_ptr = 0;
    let mut two_ptr = 0;
    let mut buffer = Vec::with_capacity(one.len() + two.len());

    if one.is_empty() {
        return two;
    } else if two.is_empty() {
        return one;
    }

    // loop will exhaust as soon as one reaches the limit otherwise they will be out of bounds.
    // using the `||` is the `or` operator and we would move out of bounds for the pointers.
    while one_ptr < one.len() && two_ptr < two.len() {
        if one[one_ptr] < two[two_ptr] {
            buffer.push(one[one_ptr]);
            one_ptr += 1;
        } else {
            buffer.push(two[two_ptr]);
            two_ptr += 1;
        }
    }

    // add the left over array from the other vector
    if one_ptr < one.len() {
        buffer.extend_from_slice(&one[one_ptr..]);
    }
    if two_ptr < two.len() {
        buffer.extend_from_slice(&two[two_ptr..]);
    }

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_both_empty() {
        let result = merge_arrays(vec![], vec![]);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn merge_first_empty() {
        let result = merge_arrays(vec![], vec![1, 2, 3]);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn merge_second_empty() {
        let result = merge_arrays(vec![1, 2, 3], vec![]);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn merge_no_overlap() {
        let result = merge_arrays(vec![1, 2, 3], vec![4, 5, 6]);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn merge_full_overlap() {
        let result = merge_arrays(vec![1, 3, 5], vec![2, 4, 6]);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn merge_with_duplicates() {
        let result = merge_arrays(vec![1, 2, 2, 3], vec![2, 3, 4]);
        assert_eq!(result, vec![1, 2, 2, 2, 3, 3, 4]);
    }

    #[test]
    fn merge_with_negatives() {
        let result = merge_arrays(vec![-5, -2, 0], vec![-3, -1, 2]);
        assert_eq!(result, vec![-5, -3, -2, -1, 0, 2]);
    }

    #[test]
    fn merge_large_values() {
        let result = merge_arrays(vec![i32::MIN, 0], vec![i32::MAX]);
        assert_eq!(result, vec![i32::MIN, 0, i32::MAX]);
    }

    #[test]
    fn merge_identical_arrays() {
        let result = merge_arrays(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, vec![1, 1, 2, 2, 3, 3]);
    }

    #[test]
    fn merge_one_element_each() {
        let result = merge_arrays(vec![2], vec![1]);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn merge_longer_lists() {
        let one = (1..=1000).step_by(2).collect::<Vec<_>>(); // odd numbers
        let two = (2..=1000).step_by(2).collect::<Vec<_>>(); // even numbers
        let result = merge_arrays(one.clone(), two.clone());
        let mut expected = one;
        expected.extend(two);
        expected.sort();
        assert_eq!(result, expected);
    }
}
