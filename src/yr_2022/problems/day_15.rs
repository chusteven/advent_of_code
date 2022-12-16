use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

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
    let (sensors, beacons) = parse_input(lines);

    let debug = false;
    let max_coord: i32 = if !debug { 4_000_000 } else { 20 };
    let mut taken_cords: HashMap<(i32, i32), char> = HashMap::new();

    sensors.iter().zip(beacons.iter()).for_each(|(s, b)| {
        let (sx, sy) = s;
        let (bx, by) = b;

        if let Entry::Vacant(v) = taken_cords.entry(*s) {
            v.insert('S');
        };

        if let Entry::Vacant(v) = taken_cords.entry(*b) {
            v.insert('B');
        };

        let manhattan_dist = (sx - bx).abs() + (sy - by).abs();

        // The problem statement is: how to mark all places that a beacon cannot
        // possibly be and very quickly...
        // Come to think of it, another way I could do it is simply inside a for
        // loop

        println!(
            "Starting to search from {:?} with MD of {:?}",
            s, manhattan_dist
        );
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        queue.push_back(*s);
        while !queue.is_empty() {
            let m = queue.pop_front().unwrap();
            for (dx, dy) in DIRECTIONS {
                let next = (m.0 + dx, m.1 + dy);
                let cur_manhattan_dist = (next.0 - sx).abs() + (next.1 - sy).abs();
                if next.0 < 0
                    || next.1 < 0
                    || next.0 > max_coord as i32
                    || next.1 > max_coord as i32
                    || cur_manhattan_dist > manhattan_dist
                    || taken_cords.contains_key(&next)
                {
                    continue;
                }
                if let Entry::Vacant(v) = taken_cords.entry(next) {
                    v.insert('#');
                }
                queue.push_back(next);
            }
        }
        println!("Finished search!");
    });

    // println!("{:?}", taken_cords);
    for i in 0..max_coord {
        for j in 0..max_coord {
            let coordinate = (i, j);
            match taken_cords.entry(coordinate) {
                Entry::Vacant(_) => {
                    return i * 4_000_000 + j;
                }
                Entry::Occupied(_) => (),
            };
        }
    }
    0
}

#[allow(dead_code)]
pub fn solution_2_try_2(input_file: &str) -> i32 {
    let lines = utils::read_file(input_file).unwrap();
    let (sensors, beacons) = parse_input(lines);

    let debug = true;
    let max_coord: i32 = if !debug { 4_000_000 } else { 20 };

    for i in 0..max_coord {
        for j in 0..max_coord {
            let mut should_continue = false;

            // This is somewhat fast, but when doing a n^2 search, then
            // each iteration's time starts adding up...
            sensors.iter().zip(beacons.iter()).for_each(|(s, b)| {
                let (sx, sy) = s;
                let (bx, by) = b;

                // Between sensor and beacon there are no other beacons
                // So if this point is between then, then it's reachable
                // and we can immediately continue
                // If not, then we know we are in a special position
                // where we must be the single beacon

                let manhattan_dist = (sx - bx).abs() + (sy - by).abs();
                let md_from_cur = (i - sx).abs() + (j - sy).abs();
                if md_from_cur <= manhattan_dist {
                    if !should_continue {
                        should_continue = true;
                        return;
                    }
                }
            });
            if should_continue {
                continue;
            }
            println!("Found the answer at ({i}, {j})");
            return i * 4_000_000 + j;
        }
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

    let mut ans = 0;
    for i in row {
        if i != '.' {
            ans += 1;
        }
    }
    ans
}

fn get_maxes(sensors: &[(i32, i32)], beacons: &[(i32, i32)]) -> (i32, i32) {
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
