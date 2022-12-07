use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

use crate::yr_2022::problems::utils;

pub fn solution_2(filepath: &str) -> i32 {
    let _lines = utils::read_file(filepath).unwrap();
    0
}

///
/// Mean to approximate also need to keep track of
/// what each `cur_dir` contains; something like:
///
/// ```json
/// {
///     "path_name": {
///         "total_size": 0,
///         "dirs": ["", ""],
///     }
/// }
/// ```
#[derive(Debug)]
struct Path {
    total_size: i32,
    dirs: Vec<String>,
}

pub fn solution_1(filepath: &str) -> i32 {
    let lines = utils::read_file(filepath).unwrap();
    let mut is_listing: bool = false;
    let mut cur_dir = "";
    let mut dirs = vec![];
    let mut filesystem: HashMap<&str, Path> = HashMap::new();

    for line in lines.iter() {
        let raw_command: &str;
        if line.starts_with("$ ") {
            raw_command = line[2..line.len()].trim();
            if raw_command.starts_with("cd") {
                let nav_path = &raw_command[2..raw_command.len()].trim();
                if *nav_path == ".." {
                    cur_dir = dirs.pop().unwrap();
                } else {
                    dirs.push(nav_path);
                    cur_dir = nav_path;
                }
                is_listing = false;
            } else {
                is_listing = true
            }
        }
        if is_listing {
            let parts: Vec<&str> = line.split(' ').collect();
            let first = parts[0];
            if first.starts_with("dir") {
                let dir = parts[1].trim();
                match filesystem.entry(cur_dir) {
                    Vacant(e) => {
                        e.insert(Path {
                            total_size: 0,
                            dirs: vec![dir.to_string()],
                        });
                    }
                    Occupied(mut e) => {
                        let p = e.get_mut();
                        let dirs = &mut p.dirs;
                        dirs.push(dir.to_string());
                    }
                }
            } else {
                let size = first.trim();
                let _file = parts[1].trim();
                match filesystem.entry(cur_dir) {
                    Vacant(e) => {
                        e.insert(Path {
                            total_size: 0,
                            dirs: vec![],
                        });
                    }
                    Occupied(mut e) => {
                        let p = e.get_mut();
                        p.total_size += size.to_string().parse::<i32>().unwrap();
                    }
                }
            }
        }
    }
    // I think it makes sense to recursive do stuff at the end;
    // So we need to find all the bottom level directories and then
    // update those one up from them
    let mut dirs_to_update = vec![];
    filesystem
        .iter()
        .filter(|(_, p)| p.dirs.is_empty())
        .for_each(|(k, _)| {
            dirs_to_update.push(*k);
        });
    update_filesystem(&mut filesystem, &mut dirs_to_update);
    println!("{:?}", dirs_to_update);
    println!("{:?}", filesystem);
    0
}

fn update_filesystem(filesystem: &mut HashMap<&str, Path>, dirs_to_update: &mut Vec<&str>) {
    // While loop
    // Inside each iteration,
    // Investigate one level deeper
    // Add up the total size of all those
    // To its own total_size
    // Then set to parents
    // Do it until get to "/"
}
