use std::cmp::Ordering;

#[test]
fn test_day07_1() {
    let data = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(day07_1(data), 6440);
}

pub fn day07_1(data: &str) -> u64 {
    let mut entries: Vec<Vec<&str>> = data
        .lines()
        .map(|x| x.split(" ").collect::<Vec<_>>())
        .collect();
    entries.sort_by(|x, y| compare_hands((get_type(x[0]), x[0]), (get_type(y[0]), y[0])));
    entries
        .iter()
        .map(|x| x[1])
        .enumerate()
        .map(|(i, x)| (i as u64 + 1) * x.parse::<u64>().expect("not a number"))
        .sum()
}

fn get_type(hand: &str) -> u8 {
    let mut _hand: Vec<char> = hand.chars().collect();
    _hand.sort();
    let levels: Vec<bool> = _hand
        .iter()
        .take(4)
        .zip(_hand.iter().skip(1))
        .map(|(x, y)| x == y)
        .collect();
    if (&levels).iter().all(|x| *x == true) {
        return 6;
    } else if levels == vec![true, true, true, false] || levels == vec![false, true, true, true] {
        return 5;
    } else if levels == vec![true, true, false, true] || levels == vec![true, false, true, true] {
        return 4;
    } else if levels == vec![true, true, false, false]
        || levels == vec![false, true, true, false]
        || levels == vec![false, false, true, true]
    {
        return 3;
    } else if levels.iter().filter(|x| **x).count() == 2 {
        return 2;
    } else if levels.iter().any(|x| *x == true) {
        return 1;
    } else {
        return 0;
    }
}

fn compare_hands(hand1: (u8, &str), hand2: (u8, &str)) -> Ordering {
    let court = |x: char| {
        if x == 'A' {
            14
        } else if x == 'K' {
            13
        } else if x == 'Q' {
            12
        } else if x == 'J' {
            11
        } else if x == 'T' {
            10
        } else {
            x.to_digit(10).unwrap()
        }
    };
    if hand1.0 != hand2.0 {
        return hand1.0.cmp(&hand2.0);
    } else {
        for (i, j) in hand1.1.chars().zip(hand2.1.chars()) {
            if i != j {
                return court(i).cmp(&court(j));
            }
        }
    }
    Ordering::Equal
}

#[test]
fn test_day07_2() {
    let data = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(day07_2(data), 5905);
}

fn get_type_with_joker(hand: &str) -> u8 {
    hand.chars()
        .map(|x| get_type(&hand.replace('J', &(x.to_string()))))
        .max()
        .unwrap()
}

fn compare_hands_with_joker(hand1: (u8, &str), hand2: (u8, &str)) -> Ordering {
    let court = |x: char| {
        if x == 'A' {
            14
        } else if x == 'K' {
            13
        } else if x == 'Q' {
            12
        } else if x == 'J' {
            1
        } else if x == 'T' {
            10
        } else {
            x.to_digit(10).unwrap()
        }
    };
    if hand1.0 != hand2.0 {
        return hand1.0.cmp(&hand2.0);
    } else {
        for (i, j) in hand1.1.chars().zip(hand2.1.chars()) {
            if i != j {
                return court(i).cmp(&court(j));
            }
        }
    }
    Ordering::Equal
}

pub fn day07_2(data: &str) -> u64 {
    let mut entries: Vec<Vec<&str>> = data
        .lines()
        .map(|x| x.split(" ").collect::<Vec<_>>())
        .collect();
    entries.sort_by(|x, y| {
        compare_hands_with_joker(
            (get_type_with_joker(x[0]), x[0]),
            (get_type_with_joker(y[0]), y[0]),
        )
    });
    entries
        .iter()
        .map(|x| x[1])
        .enumerate()
        .map(|(i, x)| (i as u64 + 1) * x.parse::<u64>().expect("not a number"))
        .sum()
}
