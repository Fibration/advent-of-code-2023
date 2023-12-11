use advent_of_code_2023::{daily::day09::{day09_1, day09_2}, utils::read};

fn main() {
    // let steps = read("src/data/08_steps".to_string());
    let data = read("src/data/09".to_string());
    let answer = day09_2(&data);
    println!("{answer:?}");
}
