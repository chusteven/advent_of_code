use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};

use crate::yr_2022::problems::utils;

pub fn solution_2(filepath: &str) -> i32 {
    let lines = utils::read_file(filepath).unwrap();
    let mut is_listing: bool = false;
    let cur_dir = &mut String::new();
    let mut dirs: Vec<String> = vec![];
    let mut filesystem: HashMap<String, Path> = HashMap::new();
    let mut parent_to_child: HashMap<String, String> = HashMap::new();

    for (_i, line) in lines.iter().enumerate() {
        if line.starts_with("$ ") {
            let raw_command = line[2..line.len()].trim();
            if raw_command.starts_with("cd") {
                let nav_path = &raw_command[2..raw_command.len()].trim();
                if *nav_path == ".." {
                    dirs.pop();
                } else {
                    dirs.push(nav_path.to_string());
                    parent_to_child.insert(cur_dir.to_string(), dirs.join("/"));
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
                        let mut p = e.get_mut();
                        p.total_size += size;
                    }
                }
            }
        }
    }
    recurse("/", &mut filesystem, &parent_to_child);
    let cur_dir = "/";
    let total_size = filesystem.get(cur_dir).unwrap().total_size;

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
    let mut parent_to_child: HashMap<String, String> = HashMap::new();

    for (_i, line) in lines.iter().enumerate() {
        if line.starts_with("$ ") {
            let raw_command = line[2..line.len()].trim();
            if raw_command.starts_with("cd") {
                let nav_path = &raw_command[2..raw_command.len()].trim();
                if *nav_path == ".." {
                    dirs.pop();
                } else {
                    dirs.push(nav_path.to_string());
                    parent_to_child.insert(cur_dir.to_string(), dirs.join("/"));
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
                        let mut p = e.get_mut();
                        p.total_size += size;
                    }
                }
            }
        }
    }
    recurse("/", &mut filesystem, &parent_to_child);
    filesystem
        .iter()
        .map(|(_, v)| v)
        .map(|v| v.total_size)
        .filter(|s| s <= &100_000)
        .sum()
}

#[allow(clippy::only_used_in_recursion)] // Very confused about this lint
fn recurse(
    cur_dir: &str,
    filesystem: &mut HashMap<String, Path>,
    parent_to_child: &HashMap<String, String>,
) -> i32 {
    let tmp = filesystem.get(cur_dir).unwrap().clone(); // Sigh
    if tmp.dirs.is_empty() {
        return tmp.total_size;
    }
    let mut size = 0;
    for child in &tmp.dirs {
        size += recurse(child, filesystem, parent_to_child);
    }
    let mut dir = filesystem.get_mut(cur_dir).unwrap();
    dir.total_size += size;
    dir.total_size
}
