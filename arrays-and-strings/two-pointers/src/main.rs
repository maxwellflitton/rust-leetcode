//! Here we are using two pointers to traverse lists. The two pointers are useful for having linear time O(n) instead of O(n^2).
//! We can use two pointers for the following:
//!
//! 1. reversing a string or array (loop front to back swaping the mem)
//! 2. checking for palindromes
//! 3. removing duplicates or elements
//! 4. two sum in a sorted array
//! 5. container with the most water
//! 6. merging sorted arrays
//! 7. sliding window expansion
//! 8. partitioning and rearranging arrays
//!
//! # Overall approach to two pointes
//!
//! 1. `let mut left_ptr = 0;`
//! 2. `let mut right_ptr = some_iterator.len().saturating_sub(1); // avoids underflow on empty interators`
//! 3. `while left_ptr < right_ptr`
//!
//! How we alter the pointers in the loop depends on the problem
mod merge_arrays;
mod palidrome;
mod reverse_a_string;
mod sorted_squares;
mod string_subsets;
mod two_sum;

fn main() {
    println!("Hello, world!");
}
