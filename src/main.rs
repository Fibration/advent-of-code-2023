use advent_of_code_2023::{daily::day07::{day07_1, day07_2}, utils::read};

fn main() {
    let data = read("src/data/07".to_string());
    // let answer = day07_1(&data);
    let answer = day07_2(&data);
    println!("{answer:?}");
}
