use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

use crate::yr_2022::problems::utils;

pub fn solution_2(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    for line in lines {
        //
    }
    0
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>, // Mutates from round to round
    operation: (String, i32),
    divisible_test: i32,
    outcome: (usize, usize),
}

pub fn solution_1(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    // We need to get n -- how many monkeys
    // And for each monkey we should have
    // a/ What items they start with (changes)
    // b/ Their operation (doesn't change)
    // c/ Their test (doesn't change)
    // d/ Outcome
    let mut monkeys = process_input(lines);
    let num_rounds = 20;
    let mut monkey_to_items_processed = HashMap::new();
    for _ in 0..num_rounds {
        process_round(&mut monkeys, &mut monkey_to_items_processed);
    }
    println!("{:#?}", monkey_to_items_processed);
    let mut counts = monkey_to_items_processed
        .iter()
        .map(|(_, v)| *v)
        .collect::<Vec<i32>>();
    counts.sort();
    let n = counts.len();
    counts[n - 1] * counts[n - 2]
}

fn process_round(
    monkeys: &mut HashMap<usize, Monkey>,
    monkey_to_items_processed: &mut HashMap<usize, i32>,
) {
    let mut items_for_next_monkey: HashMap<usize, Vec<i32>> = HashMap::new();
    let mut monkey_nos = monkeys.iter().map(|(k, _)| *k).collect::<Vec<usize>>();
    monkey_nos.sort();
    for monkey_no in monkey_nos {
        let monkey = monkeys.get(&monkey_no).unwrap();
        let (op, op_val) = &monkey.operation;
        let divisible_test = monkey.divisible_test;
        let (t_outcome, f_outcome) = monkey.outcome;
        for item in &monkey.items {
            let mut new_worry_level = {
                match op.as_str() {
                    "*" => item * op_val,
                    "/" => item / op_val,
                    "+" => item + op_val,
                    "-" => item - op_val,
                    "^2" => item * item,
                    _ => panic!("Unknown operation"),
                }
            };
            new_worry_level /= 3;
            // TODO: DRY this code
            if new_worry_level % divisible_test == 0 {
                let next_monkey = monkeys.get_mut(&t_outcome).unwrap();
                // t_outcome
            } else {
                // f_outcome
            }
        }
        // match monkey_to_items_processed.entry(monkey_no) {
        //     Vacant(e) => {
        //         e.insert(monkey.items.len() as i32);
        //     }
        //     Occupied(mut e) => {
        //         let x = e.get_mut();
        //         *x = *x + (monkey.items.len() as i32)
        //     }
        // }
    }
    // println!("Beg of round: {:#?}", monkeys);
    for (monkey_no, items) in items_for_next_monkey {
        let e = monkeys.get_mut(&monkey_no).unwrap();
        e.items = items;
    }
    // println!("End of round: {:#?}", monkeys);
}

fn process_input(lines: Vec<String>) -> HashMap<usize, Monkey> {
    let mut monkeys = HashMap::new();
    let mut monkey_no = 0;
    let mut items: Vec<i32> = vec![];
    let mut operation = (String::from(""), 0);
    let mut divisible_test = 0;
    let mut true_outcome = 0;
    let mut false_outcome = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("Monkey") {
            monkey_no = line
                .split(' ')
                .into_iter()
                .nth(1)
                .unwrap()
                .replace(":", "")
                .trim()
                .parse::<usize>()
                .unwrap();
        } else if line.starts_with("  Starting items:") {
            items = line
                .split(':')
                .into_iter()
                .nth(1)
                .unwrap()
                .split(',')
                .into_iter()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
        } else if line.starts_with("  Operation:") {
            let parts = line
                .split(" new = old ")
                .into_iter()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .nth(1)
                .unwrap()
                .split(' ')
                .into_iter()
                .collect::<Vec<&str>>();
            if parts[1] == "old" {
                operation = ("^2".to_string(), 0);
            } else {
                operation = (parts[0].to_string(), parts[1].parse::<i32>().unwrap());
            }
        } else if line.starts_with("  Test:") {
            divisible_test = line.split("divisible by ").collect::<Vec<&str>>()[1]
                .trim()
                .parse::<i32>()
                .unwrap();
        } else if line.starts_with("    If true:") {
            true_outcome = line
                .split(" throw to monkey ")
                .into_iter()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();
        } else if line.starts_with("    If false:") {
            false_outcome = line
                .split(" throw to monkey ")
                .into_iter()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();
        }
        if line.trim().is_empty() || i == lines.len() - 1 {
            monkeys.insert(
                monkey_no,
                Monkey {
                    items: items.clone(),
                    operation: operation.clone(),
                    divisible_test,
                    outcome: (true_outcome, false_outcome),
                },
            );
        }
    }
    monkeys
}
