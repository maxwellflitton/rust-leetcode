use std::collections::HashMap;

pub fn elite_max_number_of_balloons(text: String) -> i32 {
    let mut counts = [0; 26];

    //
    for byte in text.bytes() {
        counts[(byte - b'a') as usize] += 1;
    }

    let b = counts[(b'b' - b'a') as usize];
    let a = counts[(b'a' - b'a') as usize];
    let l = counts[(b'l' - b'a') as usize] / 2;
    let o = counts[(b'o' - b'a') as usize] / 2;
    let n = counts[(b'n' - b'a') as usize];

    b.min(a).min(l).min(o).min(n)
}

pub fn max_number_of_balloons(text: String) -> i32 {
    let mut string_map = HashMap::new();

    // construct the counts
    for i in text.chars() {
        match string_map.get_mut(&i) {
            Some(mut_ref) => {
                *mut_ref += 1;
            }
            None => {
                string_map.insert(i, 1);
            }
        }
    }

    // see which count for doubles is the lowest
    let max_mounts_l = match string_map.get(&'l') {
        None => return 0,
        Some(val) => val / 2,
    };
    let max_mounts_o = match string_map.get(&'o') {
        None => return 0,
        Some(val) => val / 2,
    };

    // set the lowest on the single
    let mut lowest: i32;
    if max_mounts_o > max_mounts_l {
        lowest = max_mounts_l;
    } else {
        lowest = max_mounts_o;
    }

    // ensure that there's enough single counts
    for i in ['b', 'a', 'n'] {
        match string_map.get(&i) {
            Some(val) => {
                if val < &lowest {
                    lowest = *val;
                }
            }
            None => return 0,
        }
    }
    lowest
}
