#[test]
fn test_parse_mapping() {
    let data = "50 98 2
52 50 48";
    let result = vec![(98, 100, 50), (50, 98, 52)];
    assert_eq!(parse_mapping(data), result);
}

fn parse_mapping(data: &str) -> Vec<(u64, u64, u64)> {
    data.lines()
        .map(|x| {
            x.split(" ")
                .map(|y| y.parse().expect("Not a number!"))
                .collect()
        })
        .map(|x: Vec<u64>| (x[1], x[1] + x[2], x[0]))
        .collect()
}

#[test]
fn test_convert() {
    let mapping: Vec<(u64, u64, u64)> = vec![(98, 100, 50), (50, 98, 52)];
    assert_eq!(convert(99, &mapping), 51);
    assert_eq!(convert(53, &mapping), 55);
    assert_eq!(convert(10, &mapping), 10);
}

fn convert(input: u64, mapping: &[(u64, u64, u64)]) -> u64 {
    for instruction in mapping {
        if input >= instruction.0 && input < instruction.1 {
            return instruction.2 + (input - instruction.0);
        }
    }
    input
}

#[test]
fn test_day05_1() {
    let input = "79 14 55 13";
    let data = "50 98 2
52 50 48
|
0 15 37
37 52 2
39 0 15
|
49 53 8
0 11 42
42 0 7
57 7 4
|
88 18 7
18 25 70
|
45 77 23
81 45 19
68 64 13
|
0 69 1
1 0 69
|
60 56 37
56 93 4";
    assert_eq!(day05_1(input, data), 35 as u64);
}

pub fn day05_1(input: &str, data: &str) -> u64 {
    let mut nums: Vec<u64> = input
        .split(" ")
        .map(|x| x.parse().expect("Not a number"))
        .collect();
    let mappings: Vec<Vec<(u64, u64, u64)>> =
        data.split("\n|\n").map(|x| parse_mapping(x)).collect();
    for mapping in mappings {
        nums = nums.iter().map(|x| convert(*x, &mapping)).collect();
        println!("{nums:?}");
    }
    *nums.iter().min().unwrap()
}

#[test]
fn test_convert_interval() {
    let mapping: Vec<(u64, u64, u64)> = vec![(98, 100, 50), (50, 98, 52)];
    assert_eq!(
        convert_interval((95, 110), &mapping),
        vec![(50, 52), (97, 100), (100, 110)]
    );
}

fn convert_interval(interval: (u64, u64), mapping: &[(u64, u64, u64)]) -> Vec<(u64, u64)> {
    let logic = |x: &(u64, u64), map: &(u64, u64, u64)| {
        if x.0 >= map.0 && x.1 <= map.1 {
            (Some((map.2 + (x.0 - map.0), map.2 + (x.1 - map.0))), None)
        } else if x.0 < map.0 && x.1 > map.1 {
            (
                Some((map.2, map.2 + (map.1 - map.0))),
                Some(vec![(x.0, map.0), (map.1, x.1)]),
            )
        } else if x.0 <= map.1 && x.1 > map.1 {
            (
                Some((map.2 + (x.0 - map.0), map.2 + (map.1 - map.0))),
                Some(vec![(map.1, x.1)]),
            )
        } else if x.0 < map.0 && x.1 >= map.0 {
            (
                Some((map.2, map.2 + (x.1 - map.0))),
                Some(vec![(x.0, map.0)]),
            )
        } else {
            (None, Some(vec![(x.0, x.1)]))
        }
    };
    let mut converted = Vec::new();
    let mut current = vec![interval];
    let mut next = Vec::new();
    for map in mapping {
        for piece in &current {
            let (new, remaining) = logic(piece, map);
            if let Some(x) = new {
                converted.push(x);
            }
            if let Some(x) = remaining {
                next.extend(x.iter());
            }
        }
        current = next.clone();
        next = Vec::new();
    }
    [converted, current].concat()
}

#[test]
fn test_day05_2() {
    let input = "79 14 55 13";
    let data = "50 98 2
52 50 48
|
0 15 37
37 52 2
39 0 15
|
49 53 8
0 11 42
42 0 7
57 7 4
|
88 18 7
18 25 70
|
45 77 23
81 45 19
68 64 13
|
0 69 1
1 0 69
|
60 56 37
56 93 4";
    assert_eq!(day05_2(input, data), 46 as u64);
}

pub fn day05_2(input: &str, data: &str) -> u64 {
    let nums: Vec<u64> = input
        .split(" ")
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    let mut intervals: Vec<(u64, u64)> = nums.chunks(2).map(|x| (x[0], x[0] + x[1])).collect();
    println!("{intervals:?}");
    let mappings: Vec<Vec<(u64, u64, u64)>> =
        data.split("\n|\n").map(|x| parse_mapping(x)).collect();
    for mapping in mappings {
        intervals = intervals
            .iter()
            .map(|x| convert_interval(*x, &mapping))
            .reduce(|acc, x| [acc, x].concat())
            .unwrap();
        println!("{intervals:?}");
    }
    intervals.iter().map(|x| x.0).min().unwrap()
}
