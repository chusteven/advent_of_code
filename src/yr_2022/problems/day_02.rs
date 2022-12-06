use std::collections::HashMap;

use crate::yr_2022::problems::utils;

#[allow(dead_code)]
pub fn solution_2(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    let mut losing = HashMap::new();
    losing.insert("rock", "scissors");
    losing.insert("scissors", "paper");
    losing.insert("paper", "rock");
    let mut winning = HashMap::new();
    winning.insert("scissors", "rock");
    winning.insert("paper", "scissors");
    winning.insert("rock", "paper");
    let mut scoring = HashMap::new();
    scoring.insert("rock", 1);
    scoring.insert("paper", 2);
    scoring.insert("scissors", 3);
    let mut mapping = HashMap::new();
    mapping.insert("A", "rock");
    mapping.insert("X", "rock");
    mapping.insert("B", "paper");
    mapping.insert("Y", "paper");
    mapping.insert("C", "scissors");
    mapping.insert("Z", "scissors");
    let mut ans = 0;
    for line in lines.iter() {
        let parts: Vec<&str> = line.split(' ').collect();
        let opp = parts[0];
        let res = parts[1];
        ans += process_round_actual(opp, res, &winning, &losing, &scoring, &mapping);
    }
    ans
}

fn process_round_actual(
    opp: &str,
    res: &str,
    winning: &HashMap<&str, &str>,
    losing: &HashMap<&str, &str>,
    scoring: &HashMap<&str, i32>,
    mapping: &HashMap<&str, &str>,
) -> i32 {
    match res {
        "X" => {
            // Lose
            let opp = mapping.get(opp).unwrap();
            let mine = losing.get(opp).unwrap();
            *scoring.get(mine).unwrap_or(&0)
        }
        "Y" => {
            // Draw
            let mine = *mapping.get(opp).unwrap();
            3 + scoring.get(mine).unwrap_or(&0)
        }
        &_ => {
            // Win
            // Implicitly "Z"
            let opp = mapping.get(opp).unwrap();
            let mine = winning.get(opp).unwrap();
            6 + scoring.get(mine).unwrap_or(&0)
        }
    }
}

#[allow(dead_code)]
pub fn solution_1(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    let mut winning = HashMap::new();
    winning.insert(("rock", "scissors"), true);
    winning.insert(("scissors", "paper"), true);
    winning.insert(("paper", "rock"), true);
    let mut scoring = HashMap::new();
    scoring.insert("rock", 1);
    scoring.insert("paper", 2);
    scoring.insert("scissors", 3);
    let mut mapping = HashMap::new();
    mapping.insert("A", "rock");
    mapping.insert("X", "rock");
    mapping.insert("B", "paper");
    mapping.insert("Y", "paper");
    mapping.insert("C", "scissors");
    mapping.insert("Z", "scissors");
    let mut ans = 0;
    for line in lines.iter() {
        let parts: Vec<&str> = line.split(' ').collect();
        let opp = parts[0];
        let mine = parts[1];
        ans += process_round(opp, mine, &winning, &scoring, &mapping);
    }
    ans
}

fn process_round(
    opp: &str,
    mine: &str,
    winning: &HashMap<(&str, &str), bool>,
    scoring: &HashMap<&str, i32>,
    mapping: &HashMap<&str, &str>,
) -> i32 {
    let opp = mapping.get(opp).unwrap();
    let mine = mapping.get(mine).unwrap();
    let my_score = scoring.get(mine).unwrap_or(&0);
    if opp == mine {
        return 3 + my_score;
    }
    if *winning.get(&(*mine, *opp)).unwrap_or(&false) {
        return 6 + my_score;
    }
    *my_score
}
