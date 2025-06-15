//! Here we calculate the highest average of a fixed size window.

pub fn maximum_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut left_ptr: usize = 0;
    let mut right_ptr: usize = 0;

    // define the meta data
    let mut highest_average = 0.0;
    let mut current_total = 0;

    // prefil the first lot
    while right_ptr < k as usize {
        current_total += nums[right_ptr];
        right_ptr += 1;
    }
    highest_average = current_total as f64 / k as f64;

    // loop through the rest of the array
    while right_ptr < nums.len() {
        current_total += nums[right_ptr];
        current_total -= nums[left_ptr];

        let cached_average = current_total as f64 / k as f64;
        if cached_average > highest_average {
            highest_average = cached_average;
        }

        left_ptr += 1;
        right_ptr += 1;
    }

    highest_average
}
