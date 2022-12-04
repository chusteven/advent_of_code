use std::collections::{HashMap, HashSet};

use crate::yr_2021::problems::utils;

#[allow(dead_code)]
pub fn solution_2(input_file: &str) -> i64 {
    let mut map = HashMap::new();
    map.insert(")".to_string(), "(".to_string());
    map.insert("]".to_string(), "[".to_string());
    map.insert("}".to_string(), "{".to_string());
    map.insert(">".to_string(), "<".to_string());

    let mut reverse_map = HashMap::new();
    reverse_map.insert("(".to_string(), ")".to_string());
    reverse_map.insert("[".to_string(), "]".to_string());
    reverse_map.insert("{".to_string(), "}".to_string());
    reverse_map.insert("<".to_string(), ">".to_string());

    let mut lookup = HashMap::new();
    lookup.insert(")".to_string(), 3);
    lookup.insert("]".to_string(), 57);
    lookup.insert("}".to_string(), 1197);
    lookup.insert(">".to_string(), 25137);

    let mut scores = HashMap::new();
    scores.insert(")".to_string(), 1);
    scores.insert("]".to_string(), 2);
    scores.insert("}".to_string(), 3);
    scores.insert(">".to_string(), 4);

    let lines = utils::read_file(input_file).unwrap();
    let mut ret_scores = vec![];
    for line in lines.iter() {
        let stack = get_valid_stacks(line, &map);
        if !stack.is_empty() {
            let score = score_stack(stack, &reverse_map, &scores);
            ret_scores.push(score);
        }
    }
    let n = ret_scores.len();
    ret_scores.sort();
    ret_scores[n / 2]
}

fn get_valid_stacks(line: &str, map: &HashMap<String, String>) -> Vec<String> {
    let mut s = vec![];

    // Opening characters
    let vals: HashSet<String> = map.values().into_iter().map(String::from).collect();
    for char in line.chars() {
        let str = char.to_string();
        if vals.contains(&str) {
            // Is opening char, add to stack and keep it moving
            s.push(str.to_string());
            continue;
        }
        match map.get(&str) {
            None => return vec![],
            // Ideally is closing character, so look to see that it matches what's on the stack
            Some(v) => match s.last() {
                None => continue, // Incomplete, so ignore for now
                Some(lst) => {
                    if *lst == *v {
                        s.pop();
                        continue;
                    }
                    return vec![];
                }
            },
        }
    }
    s
}

fn score_stack(
    mut stack: Vec<String>,
    map: &HashMap<String, String>,
    scores: &HashMap<String, i32>,
) -> i64 {
    let mut ans = 0;
    while !stack.is_empty() {
        let cur = stack.pop().unwrap();
        let completion = map.get(&cur).unwrap();
        ans = (ans * 5) + (*scores.get(completion).unwrap_or(&0) as i64);
    }
    ans
}

#[allow(dead_code)]
pub fn solution_1(input_file: &str) -> i32 {
    let mut map = HashMap::new();
    map.insert(")".to_string(), "(".to_string());
    map.insert("]".to_string(), "[".to_string());
    map.insert("}".to_string(), "{".to_string());
    map.insert(">".to_string(), "<".to_string());

    let mut lookup = HashMap::new();
    lookup.insert(")".to_string(), 3);
    lookup.insert("]".to_string(), 57);
    lookup.insert("}".to_string(), 1197);
    lookup.insert(">".to_string(), 25137);

    let lines = utils::read_file(input_file).unwrap();
    let mut ans = 0;
    for line in lines.iter() {
        let illegal_char = find_illegal_character(line, &map);
        if !illegal_char.is_empty() {
            let cur = lookup.get(&illegal_char).unwrap_or(&0);
            ans += cur;
        }
    }
    ans
}

fn find_illegal_character(line: &str, map: &HashMap<String, String>) -> String {
    let mut s = vec![];

    // Opening characters
    let vals: HashSet<String> = map.values().into_iter().map(String::from).collect();
    for char in line.chars() {
        let str = char.to_string();
        if vals.contains(&str) {
            // Is opening char, add to stack and keep it moving
            s.push(str.to_string());
            continue;
        }
        match map.get(&str) {
            None => {
                return str.to_owned();
            }
            // Ideally is closing character, so look to see that it matches what's on the stack
            Some(v) => match s.last() {
                None => continue, // Incomplete, so ignore for now
                Some(lst) => {
                    if *lst == *v {
                        s.pop();
                        continue;
                    }
                    return str.to_owned();
                }
            },
        }
    }
    String::new()
}
