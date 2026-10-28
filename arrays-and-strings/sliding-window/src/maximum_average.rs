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

pub fn simple_maximum_average(nums: Vec<i32>, k: i32) -> f64 {
    let total_nums = nums.len();

    // early exit conditions
    if total_nums == 0 {
        return 0.0;
    }

    // no point in iterating
    if total_nums <= k as usize {
        let total: i32 = nums.iter().sum();
        return total as f64 / total_nums as f64;
    }

    // because it's contigious we start with a window of 4
    let mut l_ptr = 0;
    let mut r_ptr = k as usize;
    let mut biggest_average: f64 = 0.0;

    // do the first compute to set everything
    let total: i32 = nums[l_ptr..r_ptr].iter().sum();
    biggest_average = total as f64 / k as f64;

    // move the window
    l_ptr += 1;
    r_ptr += 1;

    loop {
        // This is expensive as we're computing everytime
        let total: i32 = nums[l_ptr..r_ptr].iter().sum();

        let cached_average = total as f64 / k as f64;
        if cached_average > biggest_average {
            biggest_average = cached_average;
        }

        // check the break
        if r_ptr == total_nums {
            break;
        }

        // move the window
        l_ptr += 1;
        r_ptr += 1;
    }
    biggest_average
}
