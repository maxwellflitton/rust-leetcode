use std::collections::HashMap;

// [0,1,1,1,1,1,0,0,0]

// Balances:

// index      value      balance
// -1                    0
// 0          0         -1
// 1          1          0
// 2          1          1
// 3          1          2
// 4          1          3
// 5          1          4
// 6          0          3
// 7          0          2
// 8          0          1

// Balance 1 first appeared at index 2.

// It appears again at index 8.

// So:

// length = 8 - 2 = 6

pub fn find_max_length(nums: Vec<i32>) -> i32 {
    // key: balance, value: index
    let mut first_seen: HashMap<i32, i32> = HashMap::new();

    // Balance 0 exists before processing any elements.
    first_seen.insert(0, -1);

    let mut balance = 0;
    let mut longest = 0;

    for (index, num) in nums.into_iter().enumerate() {
        // update the balance
        if num == 0 {
            balance -= 1;
        } else {
            balance += 1;
        }

        // we ask -> have we seen this exact balance before?
        match first_seen.get(&balance) {
            Some(&first_index) => {
                // if yes, the elements between the previous index and current index have
                // a net balance of zero so we calculaute their length
                let length = index as i32 - first_index;
                longest = longest.max(length);
            }
            None => {
                first_seen.insert(balance, index as i32);
            }
        }
    }

    longest
}
