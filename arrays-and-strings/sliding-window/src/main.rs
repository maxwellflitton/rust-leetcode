//! Sliding windows seem to follow the following approach:
//!
//! # Notes
//! calculate the size of the window with:
//!
//! ```
//! right_ptr - left_ptr + 1
//! ```
//!
//! # Steps
//! 1. clean the inputs
//! 2. define the meta data we're tracking through the loop
//! 3. perform a `right_ptr < buffer.len()` to loop through the iterable
//! 4. in loop => read the data of the right pointer
//! 5. in loop => loop through increasing the left pointer until the condition is met again
//! 6. in loop =>  update meta data
//! 7. in loop =>  increase the right pointer (has to be at the end to prevent overflow)

mod best_subarray;
mod longest_binary;
mod longest_sum_array;
mod maximum_average;
mod number_of_sub_arrays;

fn main() {
    println!("Hello, world!");
}
