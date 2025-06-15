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
//!
//! # Types of loops
//! There are multiple different types of loops
//!
//! ## Blind iteration
//!
//! Blindly traverse the iterator. This might cross but will never overflow or
//! underflow due to breaking the `left_ptr < right_ptr` before reaching `0` or
//! `len()`.
//!
//! ```
//! while left_ptr < right_ptr {
//!     // do something
//!     left_ptr += 1;
//!     right_ptr -= 1;
//! }
//! ```
//!
//! This is good for reversing a string or calculating a palidrome or other problems
//! where the moving of a pointer is not conditional.
//!
//! ## Write pointer
//!
//! If there needs to be writing onto the interator then there needs to be a third pointer
//! that is a writer pointer. The loop will look like the following:
//!
//! ```
//! while left_ptr <= right_ptr  {
//!     // if the write index moves out of bounds exit the loop because the write is done
//!     if write_idx == 0 {
//!         break;
//!     }
//!     // move the write index positive or negative depending on the problem
//!     write_idx -= 1;
//! }
//! ```
//!
mod merge_arrays;
mod palidrome;
mod reverse_a_string;
mod sorted_squares;
mod string_subsets;
mod two_sum;

fn main() {
    println!("Hello, world!");
}
