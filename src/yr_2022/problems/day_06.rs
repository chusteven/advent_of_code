use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

use crate::yr_2022::problems::utils;

pub fn solution_2(filepath: &str) -> i32 {
    let lines = utils::read_file(filepath).unwrap();
    for line in lines.iter() {
        let ans = get_index_of_marker(line, 14);
        println!("ans (part 2) is: {ans}");
    }
    0
}

pub fn solution_1(filepath: &str) -> i32 {
    let lines = utils::read_file(filepath).unwrap();
    for line in lines.iter() {
        let ans = get_index_of_marker(line, 4);
        println!("ans (part 1) is: {ans}");
    }
    0
}

fn get_index_of_marker(line: &str, n: usize) -> usize {
    let mut uniqs = HashMap::new();
    let mut cnt = 0;
    for (i, c) in line.chars().enumerate() {
        match uniqs.entry(c) {
            Vacant(e) => {
                cnt += 1;
                if cnt == n {
                    return i + 1;
                }
                e.insert(i);
            }
            Occupied(e) => {
                // We need to find where it previously was
                let prv = e.remove();
                let diff = i - prv;
                maybe_remove(&mut uniqs, n - diff - 1, prv);
                cnt = diff;
                uniqs.insert(c, i);
            }
        }
    }
    0
}

fn maybe_remove(map: &mut HashMap<char, usize>, n: usize, from: usize) {
    let chars_to_remove: Vec<char> = map
        .iter()
        .filter(|(c, i)| **i < from)
        .map(|(c, _)| *c)
        .collect();
    chars_to_remove.iter().for_each(|c| {
        map.remove(c);
    })
}
