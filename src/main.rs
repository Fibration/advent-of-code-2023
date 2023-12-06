use advent_of_code_2023::{daily::day06::day06_2, utils::read};

fn main() {
    let data = read("src/data/06".to_string());
    let answer = day06_2(&data);
    println!("{answer:?}");
}
