
/// Find the missing number and it's assending
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let ideal_total = n * (n + 1) / 2;

    let actual_total: i32 = nums.iter().sum();

    // the total count will always be larger
    (ideal_total - actual_total) as i32
}
