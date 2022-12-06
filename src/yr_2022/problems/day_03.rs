use std::str;

use std::collections::{HashMap, HashSet};

use crate::yr_2022::problems::utils;

#[allow(dead_code)]
pub fn solution_2(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    let scoring = get_scoring();
    let mut ans = 0;
    let n = lines.len();
    for i in (0..n).step_by(3) {
        let cur_lines = &lines[i..i + 3];
        let char = get_overlapping_char_across_lines(cur_lines);
        ans += scoring.get(&char).unwrap_or(&0);
    }
    ans
}

fn get_overlapping_char_across_lines(lines: &[String]) -> String {
    let first_line = &lines[0];
    let mut chars = HashSet::new();
    for c in first_line.chars() {
        chars.insert(c);
    }
    for line in lines[1..].iter() {
        let mut new = HashSet::new();
        for c in line.chars() {
            if chars.contains(&c) {
                new.insert(c);
            }
        }
        chars = new; // Is this idiomatic??
    }
    // Wish I could `pop` from a `HashSet` but ¯\_(ツ)_/¯
    if let Some(c) = chars.iter().next() {
        return c.to_string();
    }
    panic!("This shouldn't have happened o.O")
}

#[allow(dead_code)]
pub fn solution_1(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    let scoring = get_scoring();
    let mut ans = 0;
    for line in lines.iter() {
        let n = line.len();
        let frst = &line[0..n / 2];
        let last = &line[n / 2..];

        let overlap = get_overlapping_char(frst, last);
        let score = scoring.get(&overlap).unwrap_or(&0);
        ans += score;
    }
    ans
}

fn get_scoring() -> HashMap<String, i32> {
    let mut scores = HashMap::new();
    for (i, c) in (b'a'..=b'z').enumerate() {
        let byte_arr = &vec![c];
        let str = str::from_utf8(byte_arr).unwrap();
        scores.insert(str.to_string(), (i + 1) as i32);
    }
    for (i, c) in (b'A'..=b'Z').enumerate() {
        let byte_arr = &vec![c];
        let str = str::from_utf8(byte_arr).unwrap();
        scores.insert(str.to_string(), (i + 27) as i32);
    }
    scores
}

fn get_overlapping_char(frst: &str, last: &str) -> String {
    let mut xoverlap = HashSet::new();
    let mut yoverlap = HashSet::new();
    for (x, y) in frst.chars().into_iter().zip(last.chars().into_iter()) {
        xoverlap.insert(x);
        yoverlap.insert(y);
        if xoverlap.contains(&y) {
            return y.to_string();
        }
        if yoverlap.contains(&x) {
            return x.to_string();
        }
    }
    "".to_string()
}
