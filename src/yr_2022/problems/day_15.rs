use std::collections::{HashSet, VecDeque};

use crate::yr_2022::problems::utils;

const DIRECTIONS: &[(i32, i32); 4] = &[
    (-1, 0), // Up
    (1, 0),  // Down
    (0, 1),  // Right
    (0, -1), // Left
];

#[allow(dead_code)]
pub fn solution_2(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    for _line in lines {
        //
    }
    0
}

///
/// We can iterate through pairs of sensors and beacons; as soon as one might
/// cross the row of interest, we can go ahead and process it.
///
#[allow(dead_code)]
pub fn solution_1(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    let (sensors, beacons) = parse_input(lines);

    let (max_x, max_y) = get_maxes(&sensors, &beacons);
    let mut row = vec!['.'; ((max_x as usize) * 4) + 1];
    let row_of_interest = if true { 2_000_000 + max_y } else { 10 + max_y };
    println!(
        "max_x is {max_x}; max_y is {max_y}; row of interest is {row_of_interest}; row is len {}",
        row.len()
    );

    sensors
        .iter()
        .zip(beacons.iter())
        .map(|(s, b)| {
            // print!("Mapping at original sensor [{:?}] and beacon [{:?}]", s, b);
            ((s.0 + max_x, s.1 + max_y), (b.0 + max_x, b.1 + max_y))
        })
        .for_each(|(s, b)| {
            let (sx, sy) = s;
            let (bx, by) = b;

            if sy == row_of_interest {
                row[sx as usize] = 'S';
            }
            if by == row_of_interest {
                row[bx as usize] = 'B';
            }

            let manhattan_dist = (sx - bx).abs() + (sy - by).abs();
            // println!(" and manhattan dist is: {}", manhattan_dist);

            if (sy - row_of_interest).abs() <= manhattan_dist {
                let diff = manhattan_dist - (sy - row_of_interest).abs();
                for i in (sx - diff)..(sx + diff) {
                    if row[i as usize] != '.' {
                        continue;
                    }
                    row[i as usize] = '#';
                }
            }
        });

    // println!("{:?}", row);
    let mut ans = 0;
    for i in row {
        if i != '.' {
            ans += 1;
        }
    }
    ans
}

fn mark_grid_at_row(
    point: &(i32, i32),
    row: &mut Vec<char>,
    max_x: &usize,
    max_y: &usize,
    manhattan_dist: &i32,
    row_of_interest: &i32,
) {
    // BFS outward; stopping condition is when Manhattan distance is farther
    // than it is now
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let start = (max_x + point.0 as usize, max_y + point.1 as usize);
    queue.push_back(start);

    while !queue.is_empty() {
        let m = queue.pop_front().unwrap();
        for dir in DIRECTIONS {
            let next = (m.0 as i32 + dir.0, m.1 as i32 + dir.1);
            if ((start.0 as i32 - next.0).abs() + (start.1 as i32 - next.1).abs()) > *manhattan_dist
            {
                continue;
            }
            let next = (next.0 as usize, next.1 as usize);
            if next.1 == *row_of_interest as usize {
                row[next.0] = '#';
            }
            queue.push_back(next);
        }
    }
}

fn get_maxes(sensors: &Vec<(i32, i32)>, beacons: &Vec<(i32, i32)>) -> (i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    sensors.iter().zip(beacons.iter()).for_each(|(s, b)| {
        if s.0 > max_x {
            max_x = s.0;
        }
        if b.0 > max_x {
            max_x = s.0;
        }
        if s.1 > max_y {
            max_y = s.1;
        }
        if b.0 > max_y {
            max_y = s.1;
        }
    });
    (max_x, max_y)
}

fn parse_input(lines: Vec<String>) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let mut sensors = vec![];
    let mut beacons = vec![];

    for line in lines {
        let parts = line.split(':');
        let mut sensor = (0, 0);
        let mut beacon = (0, 0);
        for (i, p) in parts.into_iter().enumerate() {
            if i == 0 {
                sensor = get_coordinates_from_str(p);
            } else {
                beacon = get_coordinates_from_str(p);
            }
        }
        sensors.push(sensor);
        beacons.push(beacon);
    }

    (sensors, beacons)
}

fn get_coordinates_from_str(string: &str) -> (i32, i32) {
    let parts = string.split(' ');
    let mut x = 0;
    let mut y = 0;
    for p in parts {
        let n = p.len();
        if p.contains("x=") {
            let ind = p.find("x=").unwrap();
            x = p[ind + 2..n - 1].trim().parse::<i32>().unwrap();
        } else if p.contains("y=") {
            let ind = p.find("y=").unwrap();
            y = p[ind + 2..n].trim().parse::<i32>().unwrap();
        }
    }
    (x, y)
}
