use num::integer::lcm;
use std::{clone, collections::HashMap};

#[test]
fn test_day08_1() {
    let instruction = "LLR";
    let data = "AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(day08_1(instruction, data), 6);
}

pub fn day08_1(instruction: &str, data: &str) -> usize {
    let mut steps = Vec::new();
    let mut current = "AAA".to_string();
    let _instruction = instruction.chars().cycle();
    let book: HashMap<String, (String, String)> = HashMap::from_iter(data.lines().map(|x| {
        (
            x[..3].to_string(),
            (x[7..10].to_string(), x[12..15].to_string()),
        )
    }));
    for step in _instruction {
        let options = book.get(&current).unwrap();
        if current == "ZZZ" {
            break;
        }
        steps.push(current);
        if step == 'L' {
            current = options.0.clone();
        } else {
            current = options.1.clone();
        }
    }
    steps.len()
}

#[test]
fn test_day08_2() {
    let instruction = "LR";
    let data = "11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    assert_eq!(day08_2(instruction, data), 6);
}

pub fn day08_2(instruction: &str, data: &str) -> usize {
    let _instruction = instruction.chars().cycle();
    let book: HashMap<String, (String, String)> = HashMap::from_iter(data.lines().map(|x| {
        (
            x[..3].to_string(),
            (x[7..10].to_string(), x[12..15].to_string()),
        )
    }));
    let starts: Vec<String> = book
        .keys()
        .filter(|x| x.chars().nth(2) == Some('A'))
        .map(|x| x.to_string())
        .collect();
    starts
        .iter()
        .map(|x| get_cycle_len(x.to_string(), _instruction.clone(), book.clone()))
        .reduce(|acc, x| lcm(acc, x))
        .unwrap()
}

fn get_cycle_len(
    start: String,
    instruction: impl Iterator<Item = char>,
    book: HashMap<String, (String, String)>,
) -> usize {
    let mut steps: usize = 0;
    let mut current = start;
    for step in instruction {
        if step == 'L' {
            current = book.get(&current).unwrap().0.clone();
        } else {
            current = book.get(&current).unwrap().1.clone();
        }
        steps += 1;
        if current.chars().nth(2) == Some('Z') {
            break;
        }
    }
    steps
}
