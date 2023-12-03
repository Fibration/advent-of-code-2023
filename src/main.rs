use advent_of_code_2023::{
    daily::day02::{day02_1, day02_2},
    utils::read,
};

fn main() {
    let data = read("src/data/02".to_string());
    let answer = day02_2(&data);
    println!("{answer:?}");
}
