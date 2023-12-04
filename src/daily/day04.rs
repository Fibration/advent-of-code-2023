#[test]
fn test_day04_1() {
    let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(day04_1(data), 13);
}

pub fn day04_1(data: &str) -> u32 {
    data.lines()
        .map(|x| x.split(": ").last().unwrap())
        .map(|x| x.split(" | ").collect())
        .map(|x: Vec<&str>| (x[0], x[1]))
        .map(|(x, y)| count_intersection(&get_numbers(x), &get_numbers(y)))
        .filter(|x| *x > 0)
        .map(|x| (2 as u32).pow((x - 1) as u32))
        .sum()
}

fn get_numbers(line: &str) -> Vec<u32> {
    line.split(" ")
        .filter(|x| *x != "")
        .map(|x| x.parse().expect("Not a number!"))
        .collect()
}

fn count_intersection(a: &[u32], b: &[u32]) -> usize {
    b.iter()
        .filter(|x| a.contains(x))
        .map(|x| *x)
        .collect::<Vec<u32>>()
        .len()
}

fn get_later_cards(i: usize, matches: usize) -> Vec<usize> {
    if matches == 0 {
        return Vec::new();
    }
    ((i + 1)..(i + matches + 1)).collect()
}

#[test]
fn test_day04_2() {
    let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(day04_2(data), 30);
}

pub fn day04_2(data: &str) -> u32 {
    let mut tally: Vec<u32> = data.lines().map(|_| 1).collect();
    let guide: Vec<Vec<usize>> = data
        .lines()
        .map(|x| x.split(": ").last().unwrap())
        .map(|x| x.split(" | ").collect())
        .map(|x: Vec<&str>| (x[0], x[1]))
        .map(|(x, y)| count_intersection(&get_numbers(x), &get_numbers(y)))
        .enumerate()
        .map(|(i, x)| get_later_cards(i, x))
        .collect();
    for (i, x) in guide.iter().enumerate() {
        for j in x {
            tally[*j] += tally[i]
        }
    }
    tally.iter().sum()
}
