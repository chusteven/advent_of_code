use std::{cell::RefCell, collections::HashMap};

use crate::yr_2022::problems::utils;

#[derive(Debug)]
struct Valve {
    flow_rate: i32,
    total_neighboring_rates: i32,
    next_tunnels: Vec<String>,
}

#[allow(dead_code)]
pub fn solution_2(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    for _line in lines {
        //
    }
    0
}

#[allow(dead_code)]
pub fn solution_1(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    let valves = get_valves(lines);
    println!("{:#?}", valves);
    // let pos_scores = get_pos_scores(&lines);
    0
}

fn get_valves(lines: Vec<String>) -> HashMap<String, Valve> {
    let mut map = HashMap::new();
    for line in lines {
        let parts = line
            .split(';')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let valve_and_rate = &parts[0];
        let valve = valve_and_rate.split(' ').nth(1).unwrap().to_string();
        let flow_rate = valve_and_rate
            .split('=')
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let splitter = if parts[1].contains(" lead to valves ") {
            " lead to valves "
        } else {
            "leads to valve "
        };
        let next_tunnels = &parts[1]
            .split(splitter)
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        map.insert(
            valve,
            Valve {
                flow_rate,
                total_neighboring_rates: 0,
                next_tunnels: next_tunnels.clone(),
            },
        );
    }
    let all_valves = map.iter().map(|(k, _)| k.clone()).collect::<Vec<String>>();
    for v in all_valves {
        let mut total_neighboring_rates = 0;
        let neighbors = map.get(&v).unwrap().next_tunnels.clone();
        for nv in neighbors {
            total_neighboring_rates += map.get(&nv).unwrap().flow_rate;
        }
        map.get_mut(&v).unwrap().total_neighboring_rates = total_neighboring_rates;
    }
    map
}

fn get_pos_scores(lines: &Vec<String>) -> HashMap<String, i32> {
    let mut pos_scores = HashMap::new();
    pos_scores
}
