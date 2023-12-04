use advent_of_code_2023::{daily::day04::day04_2, utils::read};

fn main() {
    let data = read("src/data/04".to_string());
    let answer = day04_2(&data);
    println!("{answer:?}");
}
