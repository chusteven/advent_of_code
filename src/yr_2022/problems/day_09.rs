use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

use crate::yr_2022::problems::utils;

#[derive(Debug)]
pub struct Tail {
    pub tail: VecDeque<(i32, i32)>,
    pub uniqs: HashSet<(i32, i32)>,
}

///
/// Ohh... I don't know about this one.
///
pub fn solution_2(filepath: &str) -> i32 {
    let k = 9;
    let lines = utils::read_file(filepath).unwrap();
    let mut dir_to_dir_map = HashMap::new();
    dir_to_dir_map.insert("R", (1, 0));
    dir_to_dir_map.insert("L", (-1, 0));
    dir_to_dir_map.insert("U", (0, 1));
    dir_to_dir_map.insert("D", (0, -1));
    let mut cur_h = (0, 0);
    let mut cur_t = (0, 0);
    let mut uniqs = HashSet::new();
    uniqs.insert(cur_t);
    let q = VecDeque::new();
    let mut tail = Tail { tail: q, uniqs };
    let mut seen = HashSet::new();
    seen.insert(cur_t);
    for (i, line) in lines.iter().enumerate() {
        let parts: Vec<&str> = line.split(' ').collect();
        let diff = dir_to_dir_map.get(parts[0]).unwrap();
        let n = parts[1].trim().parse::<i32>().unwrap();
        for j in 0..n {
            if (i + j as usize) == 0 {
                cur_h = (cur_h.0 + diff.0, cur_h.1 + diff.1);
                continue;
            }
            follow_the_head_long_tail(&mut cur_h, &mut tail, &mut cur_t, diff, &mut seen, k);
        }
    }
    seen.len() as i32
}

fn follow_the_head_long_tail(
    cur_h: &mut (i32, i32),
    tail: &mut Tail,
    cur_t: &mut (i32, i32),
    diff: &(i32, i32),
    _seen: &mut HashSet<(i32, i32)>,
    k: usize,
) {
    // FIXME: Fun, fun, fun
    // One knot is still the head of the rope and moves according to
    // the series of motions. Each knot further down the rope follows
    // the knot in front of it using the same rules as before.
    *cur_h = (cur_h.0 + diff.0, cur_h.1 + diff.1);
    let _dx = (cur_h.0 - cur_t.0).abs();
    let _dy = (cur_h.1 - cur_t.1).abs();

    let dx: i32;
    let dy: i32;
    if (cur_t == cur_h)
        || (_dx == 1 && _dy == 1)
        || (_dx == 0 && _dy == 1)
        || (_dx == 1 && _dy == 0)
    {
        dx = 0;
        dy = 0;
    } else if cur_h.0 == cur_t.0 || cur_h.1 == cur_t.1 {
        dx = diff.0;
        dy = diff.1;
    } else {
        match diff {
            (0, 1) | (0, -1) => {
                dx = cur_h.0 - cur_t.0;
                dy = diff.1;
            }
            (1, 0) | (-1, 0) => {
                dx = diff.0;
                dy = cur_h.1 - cur_t.1;
            }
            _ => panic!("This shouldn't have happened o.O"),
        }
    }
    *cur_t = (cur_t.0 + dx, cur_t.1 + dy);
    if tail.tail.len() == k {
        let popped = tail.tail.pop_back().unwrap();
        tail.uniqs.remove(&popped);
    }
    tail.tail.push_front(*cur_t);
    // Modify everything from the front to the back
    for i in (0..tail.tail.len() - 1).rev() {
        move_part(&mut tail.tail, i, diff);
    }
    if tail.tail.len() < k {
        tail.tail.push_back((0, 0))
    }
    println!("cur_t is {:?}; and tail looks like {:?}", cur_t, tail.tail);
    return;
}

fn move_part(tail: &mut VecDeque<(i32, i32)>, i: usize, diff: &(i32, i32)) {
    let prv = tail[i + 1];
    let cur = tail[i];

    let _dx = (prv.0 - cur.0).abs();
    let _dy = (prv.1 - cur.1).abs();

    let dx: i32;
    let dy: i32;
    if (cur == prv) || (_dx == 1 && _dy == 1) || (_dx == 0 && _dy == 1) || (_dx == 1 && _dy == 0) {
        dx = 0;
        dy = 0;
    } else if cur.0 == prv.0 || cur.1 == prv.1 {
        dx = diff.0;
        dy = diff.1;
    } else {
        match diff {
            (0, 1) | (0, -1) => {
                dx = prv.0 - cur.0;
                dy = diff.1;
            }
            (1, 0) | (-1, 0) => {
                dx = diff.0;
                dy = prv.1 - cur.1;
            }
            _ => panic!("This shouldn't have happened o.O"),
        }
    }
    tail[i] = (cur.0 + dx, cur.1 + dy);
}

///
/// Let's start at (0, 0) and have it be i32 so we can traverse
/// in all directions. We will need a utility method for determining
/// where the tail should go after the head moves. And we will
/// also need a `HashSet` to keep track of the locations the T
/// has been.
///
pub fn solution_1(filepath: &str) -> i32 {
    let lines = utils::read_file(filepath).unwrap();
    let mut dir_to_dir_map = HashMap::new();
    dir_to_dir_map.insert("R", (1, 0));
    dir_to_dir_map.insert("L", (-1, 0));
    dir_to_dir_map.insert("U", (0, 1));
    dir_to_dir_map.insert("D", (0, -1));
    let mut cur_h = (0, 0);
    let mut cur_t = (0, 0);
    let mut seen = HashSet::new();
    seen.insert(cur_t);
    for (i, line) in lines.iter().enumerate() {
        let parts: Vec<&str> = line.split(' ').collect();
        let diff = dir_to_dir_map.get(parts[0]).unwrap();
        let n = parts[1].trim().parse::<i32>().unwrap();
        for j in 0..n {
            if (i + j as usize) == 0 {
                cur_h = (cur_h.0 + diff.0, cur_h.1 + diff.1);
                continue;
            }
            follow_the_head(&mut cur_h, &mut cur_t, diff, &mut seen);
        }
    }
    seen.len() as i32
}

///
/// Need to move `cur_h` in `diff` direction, and for `cur_t` to follow
/// it as needed.
///
fn follow_the_head(
    cur_h: &mut (i32, i32),
    cur_t: &mut (i32, i32),
    diff: &(i32, i32),
    seen: &mut HashSet<(i32, i32)>,
) {
    // Let's move `cur_h` and then compute whether `cur_t` needs to be moved
    // If it does, then move it and add to seen
    // If it does not, noop
    *cur_h = (cur_h.0 + diff.0, cur_h.1 + diff.1);
    let _dx = (cur_h.0 - cur_t.0).abs();
    let _dy = (cur_h.1 - cur_t.1).abs();

    let dx: i32;
    let dy: i32;
    if (cur_t == cur_h)
        || (_dx == 1 && _dy == 1)
        || (_dx == 0 && _dy == 1)
        || (_dx == 1 && _dy == 0)
    {
        dx = 0;
        dy = 0;
    } else if cur_h.0 == cur_t.0 || cur_h.1 == cur_t.1 {
        dx = diff.0;
        dy = diff.1;
    } else {
        match diff {
            // If head went up, then get dx, move that direction and also up
            // If head went down, then get dx, move that direction and also down
            (0, 1) | (0, -1) => {
                dx = cur_h.0 - cur_t.0;
                dy = diff.1;
            }
            // If head went right, then get dy, move that direction and also right
            // If head went left, then get dx, move that direction and also left
            (1, 0) | (-1, 0) => {
                dx = diff.0;
                dy = cur_h.1 - cur_t.1;
            }
            _ => panic!("This shouldn't have happened o.O"),
        }
    }
    *cur_t = (cur_t.0 + dx, cur_t.1 + dy);
    seen.insert(*cur_t);
    return;
}
