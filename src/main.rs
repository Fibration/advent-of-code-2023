use advent_of_code_2023::{daily::day05::day05_2, utils::read};

fn main() {
    let input = read("src/data/05_seeds".to_string());
    let data = read("src/data/05".to_string());
    let answer = day05_2(&input, &data);
    println!("{answer:?}");
}
