/*  https://rust-book.cs.brown.edu/ch08-03-hash-maps.html#summary
    Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
    and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/

use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let mut ipt = vec![3, 7, 2, 9, 5, 3, 8, 1, 6, 4, 7, 2, 5, 9, 3, 4, 6, 8, 2, 7];
    // let mut ipt = vec![6, 2, 5, 2, 8, 9, 3, 4, 7, 6, 1, 5, 4, 3, 7, 2, 8, 6, 5, 4];
    // let mut ipt = vec![7, 5, 3, 8, 5, 6, 9, 2, 4, 5, 7, 3, 8, 6, 2, 9, 4, 3, 8, 7];

    ipt.sort();

    println!("arr: {:?}", ipt);
    println!("median: {}", median(&ipt));
    println!("mode: {:?}", mode(&ipt))
}

fn median(arr: &Vec<i32>) -> f32 {
    if arr.len() < 2 {
        return arr[0] as f32;
    }

    // x >> 1 = (x / 2) // x & 1 = (is odd)
    let i = (arr.len() >> 1) - 1;
    if arr.len() & 1 == 1 {
        return arr[i] as f32;
    }

    return ((arr[i] + arr[i + 1]) as f32) / 2.0;
}

fn mode(arr: &Vec<i32>) -> Vec<i32> {
    let mut count_map: HashMap<i32, i32> = HashMap::new();

    for val in arr {
        let count = count_map.entry(*val).or_insert(0);
        *count += 1;
    }

    let mut mode: Vec<i32> = Vec::new();
    let mut max = 0;

    for (key, val) in count_map {
        match val.cmp(&max) {
            Ordering::Equal => mode.push(key),
            Ordering::Greater => {
                mode.clear();
                mode.push(key);
                max = val;
            }
            Ordering::Less => (),
        }
    }

    return mode;
}
