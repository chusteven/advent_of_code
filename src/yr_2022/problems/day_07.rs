use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};

use crate::yr_2022::problems::utils;

pub fn solution_2(filepath: &str) -> i32 {
    let _lines = utils::read_file(filepath).unwrap();
    0
}

///
/// Meant to approximate what each `cur_dir` contains;
/// something like:
///
/// ```json
/// {
///     "path_name": {
///         "total_size": 0,
///         "dirs": ["", ""],
///     }
/// }
/// ```
#[derive(Debug, Clone)]
struct Path {
    total_size: i32,
    dirs: HashSet<String>,
}

pub fn solution_1(filepath: &str) -> i32 {
    let lines = utils::read_file(filepath).unwrap();
    let mut is_listing: bool = false;
    let cur_dir = &mut String::new();
    let mut dirs: Vec<String> = vec![];
    let mut filesystem: HashMap<String, Path> = HashMap::new();
    let mut child_to_parent: HashMap<String, String> = HashMap::new();

    for (_i, line) in lines.iter().enumerate() {
        if line.starts_with("$ ") {
            let raw_command = line[2..line.len()].trim();
            if raw_command.starts_with("cd") {
                let nav_path = &raw_command[2..raw_command.len()].trim();
                if *nav_path == ".." {
                    dirs.pop();
                } else {
                    dirs.push(nav_path.to_string());
                    child_to_parent.insert(dirs.join("/"), cur_dir.to_string());
                }
                *cur_dir = dirs.join("/");
                is_listing = false;
            } else {
                is_listing = true
            }
        }
        if is_listing {
            if line.contains("$ ls") {
                continue;
            }
            let parts: Vec<&str> = line.split(' ').collect();
            let first = parts[0];
            if first.starts_with("dir") {
                let raw_dir = parts[1].trim();
                let mut dirs_clone = dirs.clone();
                dirs_clone.push(raw_dir.to_string());
                // FIXME: It only contains the directories seen so far
                let dir = dirs_clone.join("/");
                match filesystem.entry(cur_dir.to_string()) {
                    Vacant(e) => {
                        let mut set = HashSet::new();
                        set.insert(dir);
                        e.insert(Path {
                            total_size: 0,
                            dirs: set,
                        });
                    }
                    Occupied(mut e) => {
                        let p = e.get_mut();
                        let tmp_dirs = &mut p.dirs;
                        tmp_dirs.insert(dir);
                    }
                }
            } else {
                // File size
                let size = first.trim().to_string().parse::<i32>().unwrap();
                match filesystem.entry(cur_dir.to_string()) {
                    Vacant(e) => {
                        e.insert(Path {
                            total_size: size,
                            dirs: HashSet::new(),
                        });
                    }
                    Occupied(mut e) => {
                        let p = e.get_mut();
                        p.total_size += size;
                    }
                }
            }
        }
    }
    // FIXME: Becuase of L:64, I think I need to do top-down traversal
    // (recursion), rather than bottoms-up siiiiigh

    // I think it makes sense to recursive do stuff at the end;
    // So we need to find all the bottom level directories and then
    // update those one up from them
    let mut dirs_to_update = vec![];
    filesystem
        .iter()
        .filter(|(_, p)| p.dirs.is_empty())
        .for_each(|(k, _)| {
            dirs_to_update.push(k.to_string());
        });
    // println!("{:?}", child_to_parent);
    // println!("{:?}", filesystem);
    // println!("{:?}", dirs_to_update);
    update_filesystem_bottoms_up(filesystem, dirs_to_update, child_to_parent);
    0
}

///
/// While loop
/// Inside each iteration,
/// Investigate one level deeper
/// Add up the total size of all those
/// To its own total_size
/// Then set to parents
/// Do it until get to "/"
///
fn update_filesystem_bottoms_up(
    mut filesystem: HashMap<String, Path>,
    mut dirs_to_update: Vec<String>,
    child_to_parent: HashMap<String, String>,
) {
    let mut seen = HashSet::new();
    let filesystem = &mut filesystem;
    let dirs_to_update = &mut dirs_to_update;
    dirs_to_update.iter().for_each(|d| {
        seen.insert(d.to_string());
    });
    loop {
        let mut new_dirs_to_update = HashSet::new();
        for dir in &*dirs_to_update {
            let dir = &dir[..];
            if let Some(parent) = child_to_parent.get(dir) {
                let child_size = filesystem.get(dir).unwrap().total_size;
                let parent_path = filesystem.get(parent).unwrap().clone(); // Omg...
                for d in parent_path.dirs {
                    if seen.contains(&d) {
                        continue;
                    }
                    seen.insert(d.to_string());
                    new_dirs_to_update.insert(d);
                }
                let mut tmp = filesystem.clone();
                tmp.get_mut(parent).unwrap().total_size += child_size;
                *filesystem = tmp;
            }
        }
        *dirs_to_update = new_dirs_to_update
            .iter()
            .map(|s| (*s).to_string())
            .collect::<Vec<String>>();
        if dirs_to_update.is_empty() {
            break;
        }
    }

    // For debugging
    let mut sorted = vec![];
    for (k, v) in filesystem.iter() {
        sorted.push((k, v));
    }
    sorted.sort_by(|(a, _), (b, _)| a.cmp(b));
    println!("{:#?}", sorted);

    let ans = filesystem
        .iter()
        .map(|(_, v)| v.total_size)
        .filter(|v| *v <= 100_000)
        .sum::<i32>();
    println!("ans (part i) is: {ans}");
}
