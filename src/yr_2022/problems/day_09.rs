use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

use crate::yr_2022::problems::utils;

/// When ready
// R 5
// U 8
// L 8
// D 3
// R 17
// D 10
// L 25
// U 20

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
    let mut tail = VecDeque::new();
    tail.push_front(cur_h);
    let mut seen = HashSet::new();
    seen.insert(cur_h);

    for (i, line) in lines.iter().enumerate() {
        let parts: Vec<&str> = line.split(' ').collect();
        let diff = dir_to_dir_map.get(parts[0]).unwrap();
        let n = parts[1].trim().parse::<i32>().unwrap();
        for j in 0..n {
            if (i + (j as usize)) == 0 {
                cur_h = (cur_h.0 + diff.0, cur_h.1 + diff.1);
                continue;
            }
            follow_the_head_long_tail(&mut cur_h, &mut tail, diff, &mut seen, k);
        }
    }
    seen.len() as i32
}

///
/// `cur_h` has been moved; now we just need `tail` to follow it.
/// We need to pop one at a time from the front and, for each, follow
/// the part that was popped before it (or, for the first time, it
/// needs to follow the head). And then, if the tail is less than
/// length 9, `push_back((0, 0))`.
///
/// Notes to self: This method should do stuff --
/// 1. Pop off front -- `pop_front()`
/// 2. How does this compare to `cur_h`?
/// 3. Move it accordingly -- can send to `follow_the_head`
///     - `cur_h`, call popped `cur_t`, need `diff`, send in `seen`
/// 4. Pop off the rest, move those accordingly
///     - ... maybe `diff` changes...
///
fn follow_the_head_long_tail(
    cur_h: &mut (i32, i32),
    tail: &mut VecDeque<(i32, i32)>,
    diff: &(i32, i32),
    seen: &mut HashSet<(i32, i32)>,
    k: usize,
) {
    let mut cur_t = tail.pop_front().unwrap();

    // IMPORTANT: This ended up being the thing that f***ed me; I was
    // adding to `seen` too often :( :( :(
    follow_the_head(cur_h, &mut cur_t, diff, seen, false);
    tail.push_front(cur_t);
    for i in 0..(tail.len() - 1) {
        let _cur_h = tail[i];
        let mut _cur_t = tail[i + 1];

        let _dx = (_cur_h.0 - _cur_t.0).abs();
        let _dy = (_cur_h.1 - _cur_t.1).abs();

        let dx: i32;
        let dy: i32;

        // Three cases:
        // (1) went somewhere, but still touching
        // (2) went somewhere V or H, but no longer touching
        // (3) went somewhere D, but no longer touching

        // FIXME: Can make the below method simply to follow
        // this model as well. Or probably just refactor the
        // entire damn thing...

        // (1)
        if (_cur_t == _cur_h)
            || (_dx == 1 && _dy == 1)
            || (_dx == 0 && _dy == 1)
            || (_dx == 1 && _dy == 0)
        {
            dx = 0;
            dy = 0;
        }
        // (2)
        else if (_dx == 2 && _dy == 0) || (_dy == 2 && _dx == 0) {
            if _dx == 2 {
                dx = if _cur_h.0 - _cur_t.0 > 1 { 1 } else { -1 };
                dy = 0;
            } else {
                dx = 0;
                dy = if _cur_h.1 - _cur_t.1 > 1 { 1 } else { -1 };
            }
        }
        // (3)
        else {
            dx = if _cur_h.0 - _cur_t.0 > 0 { 1 } else { -1 };
            dy = if _cur_h.1 - _cur_t.1 > 0 { 1 } else { -1 };
        }
        _cur_t = (_cur_t.0 + dx, _cur_t.1 + dy);
        tail[i + 1] = _cur_t;
    }
    if tail.len() < k {
        if *tail.back().unwrap() != (0, 0) {
            tail.push_back((0, 0));
        }
    } else {
        seen.insert(*tail.back().unwrap());
    }
}

// fn paint_board(cur_h: &mut (i32, i32), tail: &VecDeque<(i32, i32)>) {}

fn _ref_follow_the_head(
    cur_h: &mut (i32, i32),
    cur_t: &mut (i32, i32),
    diff: &(i32, i32),
    seen: &mut HashSet<(i32, i32)>,
) {
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
            follow_the_head(&mut cur_h, &mut cur_t, diff, &mut seen, true);
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
    should_add_to_seen: bool,
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
    if !should_add_to_seen {
        return;
    }
    seen.insert(*cur_t);
}
