use std::str::FromStr;

use crate::yr_2022::problems::utils;

#[allow(dead_code)]
pub fn solution_2(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    let mut ans = 0;
    for line in lines.iter() {
        let ranges: Vec<&str> = line.split(',').collect();
        if contains_any_overlap(ranges) {
            ans += 1;
        }
    }
    ans
}

fn contains_any_overlap(ranges: Vec<&str>) -> bool {
    let frst = ranges[0];
    let scnd = ranges[1];
    let (min_1, max_1) = get_min_max_of_range(frst);
    let (min_2, max_2) = get_min_max_of_range(scnd);
    if (max_1 >= min_2 && min_2 >= min_1) || (max_2 >= min_1 && min_1 >= min_2) {
        return true;
    }
    false
}

#[allow(dead_code)]
pub fn solution_1(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    let mut ans = 0;
    for line in lines.iter() {
        let ranges: Vec<&str> = line.split(',').collect();
        if contains_full_overlap(ranges) {
            ans += 1;
        }
    }
    ans
}

fn contains_full_overlap(ranges: Vec<&str>) -> bool {
    let frst = ranges[0];
    let scnd = ranges[1];
    let (min_1, max_1) = get_min_max_of_range(frst);
    let (min_2, max_2) = get_min_max_of_range(scnd);
    if (min_1 <= min_2 && max_1 >= max_2) || (min_2 <= min_1 && max_2 >= max_1) {
        return true;
    }
    false
}

fn get_min_max_of_range(range: &str) -> (i32, i32) {
    let parts: Vec<i32> = range
        .split('-')
        .map(|d| <i32 as FromStr>::from_str(d).unwrap())
        .collect();
    (parts[0], parts[1])
}
