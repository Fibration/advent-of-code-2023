fn convert_to_digit(x: &str) -> Vec<u32> {
    x.chars()
        .filter(|x| x.is_numeric())
        .map(|x| x.to_digit(10).unwrap())
        .collect()
}

pub fn day01_1(data: &str) -> u32 {
    data.lines()
        .map(convert_to_digit)
        .map(|x: Vec<u32>| x[0] * 10 + x[x.len() - 1])
        .sum()
}

pub fn day01_2(data: &str) -> u32 {
    let replace_closure = |x: &str| {
        x.replace("eightwo", "82")
            .replace("eighthree", "83")
            .replace("twone", "21")
            .replace("sevenine", "79")
            .replace("nineight", "98")
            .replace("threeight", "38")
            .replace("fiveight", "58")
            .replace("oneight", "18")
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9")
    };
    data.lines()
        .map(replace_closure)
        .map(|x| convert_to_digit(&x))
        .map(|x: Vec<u32>| x[0] * 10 + x[x.len() - 1])
        .sum()
}

#[test]
fn test_day01_1() {
    let data = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    assert_eq!(day01_1(data), 142);
}

#[test]
fn test_day01_2() {
    let data = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    assert_eq!(day01_2(data), 281);
}
