use advent_of_code_2023::{
    daily::day01::{day01_1, day01_2},
    utils::read,
};

fn main() {
    let data = read("src/data/01_1".to_string());
    let answer = day01_2(&data);
    println!("{answer:?}");
}
