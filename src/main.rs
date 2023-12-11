use advent_of_code_2023::{daily::day10::day10_2, utils::read};

fn main() {
    // let steps = read("src/data/08_steps".to_string());
    let data = read("src/data/10".to_string());
    let answer = day10_2((0, 1), &data);
    println!("{answer:?}");
}
