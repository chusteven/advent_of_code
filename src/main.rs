mod yr_2021;
mod yr_2022;

use std::time::Instant;

fn invoke_solutions(
    year: &str,
    day: &str,
    fn1: impl Fn(&str) -> i32,
    fn2: impl Fn(&str) -> i32,
    debug_only: bool,
) {
    let play = format!("./src/yr_{year}/test_cases/day_{day}_play.txt");
    let real = format!("./src/yr_{year}/test_cases/day_{day}_real.txt");
    let ans1 = fn1(&play);
    println!("ans1 is {:?}", ans1);

    if !debug_only {
        let now = Instant::now();
        let ans1 = fn1(&real);
        let elapsed = now.elapsed();
        println!("ans1 is {:?}; time elapsed: {:?}", ans1, elapsed);
    }

    let ans2 = fn2(&play);
    println!("ans2 is {:?}", ans2);
    if !debug_only {
        let now = Instant::now();
        let ans2 = fn2(&real);
        let elapsed = now.elapsed();
        println!("ans2 is {:?}; time elapsed: {:?}", ans2, elapsed);
    }
}

#[allow(dead_code)]
fn invoke_solutions_i64(
    year: &str,
    day: &str,
    fn1: impl Fn(&str) -> i64,
    fn2: impl Fn(&str) -> i64,
    debug_only: bool,
) {
    let play = format!("./src/yr_{year}/test_cases/day_{day}_play.txt");
    let real = format!("./src/yr_{year}/test_cases/day_{day}_real.txt");
    let ans1 = fn1(&play);
    println!("ans1 is {:?}", ans1);

    if !debug_only {
        let now = Instant::now();
        let ans1 = fn1(&real);
        let elapsed = now.elapsed();
        println!("ans1 is {:?}; time elapsed: {:?}", ans1, elapsed);
    }

    let ans2 = fn2(&play);
    println!("ans2 is {:?}", ans2);
    if !debug_only {
        let now = Instant::now();
        let ans2 = fn2(&real);
        let elapsed = now.elapsed();
        println!("ans2 is {:?}; time elapsed: {:?}", ans2, elapsed);
    }
}

fn main() {
    invoke_solutions(
        "2022",
        "14",
        yr_2022::problems::day_14::solution_1,
        yr_2022::problems::day_14::solution_2,
        false,
    );
}
