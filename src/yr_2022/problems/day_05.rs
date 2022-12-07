use regex::Regex;
use std::collections::hash_map::Entry::{Occupied, Vacant};

use std::collections::HashMap;

use crate::yr_2022::problems::utils;

#[allow(dead_code)]
pub fn solution_2(filepath: &str) -> i32 {
    let lines = utils::read_file(filepath).unwrap();
    let (mut stacks, directions) = process_input(lines);
    for (num_to_move, from, to) in directions.iter() {
        let f = stacks.get_mut(from).unwrap();
        let n = f.len();

        // Idk how `drain` and `append` methods work, but it seems like there
        // shouldn't need for x to be declared as mutable...
        let mut x: Vec<char> = f.drain(n - num_to_move..n).collect();
        let t = stacks.get_mut(to).unwrap();
        t.append(&mut x);
    }
    let mut ans = vec![];
    let n = stacks.len();
    for i in 1..=n {
        let v = stacks.get(&i).unwrap();
        ans.push(v[v.len() - 1]);
    }
    let ans: String = ans.into_iter().collect();
    println!("{ans}");
    0
}

#[allow(dead_code)]
pub fn solution_1(filepath: &str) -> i32 {
    let lines = utils::read_file(filepath).unwrap();
    let (mut stacks, directions) = process_input(lines);
    for (num_to_move, from, to) in directions.iter() {
        for _ in 0..*num_to_move {
            let f = stacks.get_mut(from).unwrap();
            let x = f.pop().unwrap();
            let t = stacks.get_mut(to).unwrap();
            t.push(x);
        }
    }
    let mut ans = vec![];
    let n = stacks.len();
    for i in 1..=n {
        let v = stacks.get(&i).unwrap();
        ans.push(v[v.len() - 1]);
    }
    let ans: String = ans.into_iter().collect();
    println!("{ans}");
    0
}

fn process_input(lines: Vec<String>) -> (HashMap<usize, Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut processing_stacks = true;
    let mut raw_stacks = vec![];
    let mut raw_directions = vec![];
    for line in lines.iter() {
        if line.is_empty() {
            processing_stacks = false;
            continue;
        }
        if processing_stacks {
            raw_stacks.push(line.to_owned());
            continue;
        }
        raw_directions.push(line.to_owned());
    }

    let stacks = process_stacks(raw_stacks);
    let directions = process_directions(raw_directions);
    (stacks, directions)
}

fn process_stacks(raw: Vec<String>) -> HashMap<usize, Vec<char>> {
    let mut map = HashMap::new();
    for line in raw.iter() {
        let n = line.len();
        let mut ind: usize = 1;
        for i in (0..n).step_by(4) {
            let val = line.chars().nth(i + 1).unwrap();
            if val.is_ascii_digit() || val.is_whitespace() {
                ind += 1;
                continue;
            }
            match map.entry(ind) {
                Vacant(e) => {
                    e.insert(vec![val]);
                }
                Occupied(mut e) => {
                    e.get_mut().push(val);
                }
            }
            ind += 1;
        }
    }
    let mut inverted_map = HashMap::new();
    // Woah didn't know could do `mut v`!
    for (k, mut v) in map.into_iter() {
        v.reverse();
        inverted_map.insert(k, v);
    }
    inverted_map
}

fn process_directions(raw: Vec<String>) -> Vec<(usize, usize, usize)> {
    let re = Regex::new(r"\d+").unwrap();
    let mut directions = vec![];
    for line in raw.iter() {
        let mut thruple = vec![];
        for matches in re.find_iter(line) {
            let x = matches.as_str().parse::<usize>().unwrap();
            thruple.push(x);
        }
        directions.push((thruple[0], thruple[1], thruple[2]))
    }
    directions
}
