use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::yr_2022::problems::utils;

#[allow(dead_code)]
pub fn solution_2(input_file: &str) -> i32 {
    let k = 3;
    let lines = utils::read_file(input_file).unwrap();
    let mut elf_no = 0;
    let mut elf_cals = HashMap::new();
    for line in lines.iter() {
        if line.is_empty() {
            elf_no += 1;
            continue;
        }
        let calories = line.parse::<i32>().unwrap();
        match elf_cals.entry(elf_no) {
            Vacant(e) => {
                e.insert(calories);
            }
            Occupied(mut e) => {
                (*e.get_mut()) += calories;
            }
        }
    }
    let mut heap = BinaryHeap::new();
    for (_, v) in elf_cals.iter() {
        let cals = *v;
        if heap.len() < k {
            heap.push(-cals);
        }
        let cmp = heap.pop().unwrap();
        if -cmp > cals {
            // Put it back
            heap.push(cmp)
        } else {
            heap.push(-cals)
        }
    }
    let mut ans = 0;
    for v in heap.iter() {
        ans += -(*v);
    }
    ans
}

#[allow(dead_code)]
pub fn solution_1(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    let mut elf_no = 0;

    // CONSIDER: Do I really need to keep this as a map? There's probably
    // an easier way to just check state at each empty line ""...
    let mut elf_cals = HashMap::new();
    let mut ans = 0;
    for line in lines.iter() {
        if line.is_empty() {
            elf_no += 1;
            continue;
        }
        let calories = line.parse::<i32>().unwrap();
        match elf_cals.entry(elf_no) {
            Vacant(e) => {
                e.insert(calories);
            }
            Occupied(mut e) => {
                (*e.get_mut()) += calories;
                if *e.get() > ans {
                    ans = *e.get();
                }
            }
        }
    }
    ans
}
