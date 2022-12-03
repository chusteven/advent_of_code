mod yr_2021;
mod yr_2022;

use std::time::Instant;

#[allow(dead_code)]
fn yr_2021_day_09() {
    let ans1 = yr_2021::problems::day_09::solution_1("./src/yr_2021/test_cases/day_09_play.txt");
    println!("ans1 is {:?}", ans1);

    let now = Instant::now();
    let ans1 = yr_2021::problems::day_09::solution_1("./src/yr_2021/test_cases/day_09_real.txt");
    println!("ans1 is {:?}", ans1);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let ans2 = yr_2021::problems::day_09::solution_2("./src/yr_2021/test_cases/day_09_play.txt");
    println!("ans2 is {:?}", ans2);

    let now = Instant::now();
    let ans2 = yr_2021::problems::day_09::solution_2("./src/yr_2021/test_cases/day_09_real.txt");
    println!("ans2 is {:?}", ans2);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[allow(dead_code)]
fn yr_2021_day_10() {
    let ans1 = yr_2021::problems::day_10::solution_1("./src/yr_2021/test_cases/day_10_play.txt");
    println!("ans1 is {:?}", ans1);
    let ans1 = yr_2021::problems::day_10::solution_1("./src/yr_2021/test_cases/day_10_real.txt");
    println!("ans1 is {:?}", ans1);

    let ans2 = yr_2021::problems::day_10::solution_2("./src/yr_2021/test_cases/day_10_play.txt");
    println!("ans2 is {:?}", ans2);
    let ans2 = yr_2021::problems::day_10::solution_2("./src/yr_2021/test_cases/day_10_real.txt");
    println!("ans2 is {:?}", ans2);
}

#[allow(dead_code)]
fn yr_2021_day_11() {
    let ans1 = yr_2021::problems::day_11::solution_1("./src/yr_2021/test_cases/day_11_play.txt");
    println!("ans1 is {:?}", ans1);
    let ans1 = yr_2021::problems::day_11::solution_1("./src/yr_2021/test_cases/day_11_real.txt");
    println!("ans1 is {:?}", ans1);

    let ans2 = yr_2021::problems::day_11::solution_2("./src/yr_2021/test_cases/day_11_play.txt");
    println!("ans2 is {:?}", ans2);
    let ans2 = yr_2021::problems::day_11::solution_2("./src/yr_2021/test_cases/day_11_real.txt");
    println!("ans2 is {:?}", ans2);
}

#[allow(dead_code)]
fn yr_2022_day_01() {
    let ans1 = yr_2022::problems::day_01::solution_1("./src/yr_2022/test_cases/day_01_play.txt");
    println!("ans1 is {:?}", ans1);
    let ans1 = yr_2022::problems::day_01::solution_1("./src/yr_2022/test_cases/day_01_real.txt");
    println!("ans1 is {:?}", ans1);

    let ans2 = yr_2022::problems::day_01::solution_2("./src/yr_2022/test_cases/day_01_play.txt");
    println!("ans2 is {:?}", ans2);
    let ans2 = yr_2022::problems::day_01::solution_2("./src/yr_2022/test_cases/day_01_real.txt");
    println!("ans2 is {:?}", ans2);
}

fn yr_2022_day_02() {
    let ans1 = yr_2022::problems::day_02::solution_1("./src/yr_2022/test_cases/day_02_play.txt");
    println!("ans1 is {:?}", ans1);
    let ans1 = yr_2022::problems::day_02::solution_1("./src/yr_2022/test_cases/day_02_real.txt");
    println!("ans1 is {:?}", ans1);

    let ans2 = yr_2022::problems::day_02::solution_2("./src/yr_2022/test_cases/day_02_play.txt");
    println!("ans2 is {:?}", ans2);
    let ans2 = yr_2022::problems::day_02::solution_2("./src/yr_2022/test_cases/day_02_real.txt");
    println!("ans2 is {:?}", ans2);
}

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
        let ans1 = fn1(&real);
        println!("ans1 is {:?}", ans1);
    }

    let ans2 = fn2(&play);
    println!("ans2 is {:?}", ans2);
    if !debug_only {
        let ans2 = fn2(&real);
        println!("ans2 is {:?}", ans2);
    }
}

fn main() {
    // yr_2021_day_09();
    // yr_2021_day_10();
    // yr_2022_day_01();
    invoke_solutions(
        "2022",
        "03",
        yr_2022::problems::day_03::solution_1,
        yr_2022::problems::day_03::solution_2,
        false,
    );
}
