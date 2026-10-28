
pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut w_buffer = vec![-1; nums.len()];
    let rad = (k * 2) + 1;
    
    // early return 
    if nums.len() < rad as usize {
        return w_buffer
    }
    if k == 0 {
        return nums
    }
    
    // setup the index
    let mut c_ptr = k as usize;
    let mut l_ptr = 0;
    let mut r_ptr = rad as usize;
    
    loop {
        let total: i64 = nums[l_ptr..r_ptr].iter().map(|&num| num as i64).sum();
        
        let average: i64 = total / rad as i64;
        w_buffer[c_ptr] = average as i32; 
        
        // check the 
        if r_ptr as usize == nums.len() {
            break
        }
        
        // increase and check
        l_ptr += 1;
        r_ptr += 1;
        c_ptr += 1;
    }
    
    // return the write buffer
    w_buffer
}