use std::collections::HashSet;

pub fn count_elements(arr: Vec<i32>) -> i32 {
    let mut total = 0;
    let mut collection = HashSet::new();

    for i in &arr {
        collection.insert(i);
    }
    for i in &arr {
        if collection.contains(&(i + 1)) {
            total += 1;
        }
    }
    total
}
