use advent_of_code_2023::{daily::day08::day08_2, utils::read};

fn main() {
    let steps = read("src/data/08_steps".to_string());
    let data = read("src/data/08".to_string());
    // let answer = day08_1(&steps, &data);
    let answer = day08_2(&steps,&data);
    println!("{answer:?}");
}
