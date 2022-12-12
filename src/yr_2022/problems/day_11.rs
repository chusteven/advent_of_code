use std::cell::RefCell;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::collections::VecDeque;

use crate::yr_2022::problems::utils;

pub fn solution_2(input_file: &str) -> i128 {
    let lines = utils::read_file(input_file).unwrap();
    let mut monkeys = process_input(lines);
    let num_rounds = 10_000;
    let mut monkey_to_items_processed = HashMap::new();
    let mut supermodule = 1;
    for (_, v) in monkeys.iter() {
        supermodule = supermodule * v.borrow().divisible_test;
    }
    for _ in 0..num_rounds {
        process_round_another_way(&mut monkeys, &mut monkey_to_items_processed, supermodule);
    }
    let mut counts = monkey_to_items_processed
        .iter()
        .map(|(_, v)| *v)
        .collect::<Vec<i128>>();
    counts.sort();
    let n = counts.len();
    counts[n - 1] * counts[n - 2]
}

fn process_round_another_way(
    monkeys: &mut HashMap<usize, RefCell<Monkey>>,
    monkey_to_items_processed: &mut HashMap<usize, i128>,
    supermodulo: i128,
) {
    let mut monkey_nos = monkeys.iter().map(|(k, _)| *k).collect::<Vec<usize>>();
    monkey_nos.sort();
    // println!("Beg of round: {:?}", monkeys);
    for monkey_no in monkey_nos {
        let mut monkey = monkeys.get(&monkey_no).unwrap().borrow_mut();
        let (op, op_val) = monkey.operation.clone();
        let divisible_test = monkey.divisible_test;
        let (t_outcome, f_outcome) = monkey.outcome;
        let items = monkey.items.clone();

        // I have no idea why this works. I don't know math that well,
        // but what I did is multiply all of the testing numbers together,
        // calling it "supermodulo", and every time a monkey inspects an
        // item, set the item's value to the item mod the supermodulo,
        // like item = item % supermodulo.

        for item in items {
            let item = item % supermodulo;
            let new_worry_level = {
                match op.as_str() {
                    "*" => item * op_val,
                    "/" => item / op_val,
                    "+" => item + op_val,
                    "-" => item - op_val,
                    "^2" => item * item,
                    _ => panic!("Unknown operation"),
                }
            };
            if new_worry_level % divisible_test == 0 {
                let mut next_monkey = monkeys.get(&t_outcome).unwrap().borrow_mut();
                next_monkey.items.push_back(new_worry_level);
            } else {
                let mut next_monkey = monkeys.get(&f_outcome).unwrap().borrow_mut();
                next_monkey.items.push_back(new_worry_level);
            }
            monkey.items.pop_front();
            match monkey_to_items_processed.entry(monkey_no) {
                Vacant(e) => {
                    e.insert(1);
                }
                Occupied(mut e) => {
                    let x = e.get_mut();
                    *x = *x + 1;
                }
            }
        }
    }
    // println!("End of round: {:?}", monkeys);
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i128>, // Mutates from round to round
    operation: (String, i128),
    divisible_test: i128,
    outcome: (usize, usize),
}

pub fn solution_1(input_file: &str) -> i128 {
    let lines = utils::read_file(input_file).unwrap();
    // We need to get n -- how many monkeys
    // And for each monkey we should have
    // a/ What items they start with (changes)
    // b/ Their operation (doesn't change)
    // c/ Their test (doesn't change)
    // d/ Outcome
    let mut monkeys = process_input(lines);
    // println!("{:#?}", monkeys);
    let num_rounds = 20;
    let mut monkey_to_items_processed = HashMap::new();
    for _i in 0..num_rounds {
        process_round(&mut monkeys, &mut monkey_to_items_processed, 3);
    }
    // println!("{:#?}", monkey_to_items_processed);
    let mut counts = monkey_to_items_processed
        .iter()
        .map(|(_, v)| *v)
        .collect::<Vec<i128>>();
    counts.sort();
    let n = counts.len();
    counts[n - 1] * counts[n - 2]
}

fn process_round(
    monkeys: &mut HashMap<usize, RefCell<Monkey>>,
    monkey_to_items_processed: &mut HashMap<usize, i128>,
    stress_divide_by: i128,
) {
    let mut monkey_nos = monkeys.iter().map(|(k, _)| *k).collect::<Vec<usize>>();
    monkey_nos.sort();
    // println!("Beg of round: {:?}", monkeys);
    for monkey_no in monkey_nos {
        let mut monkey = monkeys.get(&monkey_no).unwrap().borrow_mut();
        let (op, op_val) = monkey.operation.clone();
        let divisible_test = monkey.divisible_test;
        let (t_outcome, f_outcome) = monkey.outcome;
        let items = monkey.items.clone();
        for item in items {
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
            new_worry_level /= stress_divide_by;
            if new_worry_level % divisible_test == 0 {
                let mut next_monkey = monkeys.get(&t_outcome).unwrap().borrow_mut();
                next_monkey.items.push_back(new_worry_level);
            } else {
                let mut next_monkey = monkeys.get(&f_outcome).unwrap().borrow_mut();
                next_monkey.items.push_back(new_worry_level);
            }
            monkey.items.pop_front();
            match monkey_to_items_processed.entry(monkey_no) {
                Vacant(e) => {
                    e.insert(1);
                }
                Occupied(mut e) => {
                    let x = e.get_mut();
                    *x = *x + 1;
                }
            }
        }
    }
    // println!("End of round: {:?}", monkeys);
}

fn process_input(lines: Vec<String>) -> HashMap<usize, RefCell<Monkey>> {
    let mut monkeys = HashMap::new();
    let mut monkey_no = 0;
    let mut items: VecDeque<i128> = VecDeque::new();
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
                .map(|s| s.parse::<i128>().unwrap())
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
                operation = (parts[0].to_string(), parts[1].parse::<i128>().unwrap());
            }
        } else if line.starts_with("  Test:") {
            divisible_test = line.split("divisible by ").collect::<Vec<&str>>()[1]
                .trim()
                .parse::<i128>()
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
                RefCell::new(Monkey {
                    items: items.clone(),
                    operation: operation.clone(),
                    divisible_test,
                    outcome: (true_outcome, false_outcome),
                }),
            );
        }
    }
    monkeys
}
