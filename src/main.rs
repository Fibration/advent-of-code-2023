use advent_of_code_2023::{daily::day03::day03_2, utils::read};

fn main() {
    let data = read("src/data/03".to_string());
    let answer = day03_2(&data);
    println!("{answer:?}");
}
