use std::collections::HashMap;

#[test]
fn test_day03_1() {
    let data = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(day03_1(data), 4361);
}

pub fn day03_1(data: &str) -> u32 {
    let grid: Vec<Vec<&str>> = data
        .lines()
        .map(|line| {
            line.split("")
                .collect::<Vec<&str>>()
                .iter()
                .rev()
                .map(|x| *x)
                .filter(|x| *x != "")
                .collect::<Vec<&str>>()
        })
        .collect();

    let mut processing_number = false;
    let mut current_number: u32 = 0;
    let mut decimal: u32 = 0;
    let mut engine = false;
    let mut total = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if (col.chars().collect::<Vec<char>>()[0]).is_numeric() {
                if processing_number {
                    decimal += 1;
                    current_number += col.chars().collect::<Vec<char>>()[0].to_digit(10).unwrap()
                        * ((10 as u32).pow(decimal));
                } else if !processing_number {
                    current_number = col.chars().collect::<Vec<char>>()[0].to_digit(10).unwrap();
                    if j > 0
                        && grid[i][j - 1] != "."
                        && !grid[i][j - 1].chars().collect::<Vec<char>>()[0].is_numeric()
                    {
                        engine = true;
                    } else if j > 0
                        && i < grid.len() - 1
                        && grid[i + 1][j - 1] != "."
                        && !grid[i + 1][j - 1].chars().collect::<Vec<char>>()[0].is_numeric()
                    {
                        engine = true;
                    } else if j > 0
                        && i > 0
                        && grid[i - 1][j - 1] != "."
                        && !grid[i - 1][j - 1].chars().collect::<Vec<char>>()[0].is_numeric()
                    {
                        engine = true;
                    }
                    processing_number = true;
                }
                if i < grid.len() - 1
                    && grid[i + 1][j] != "."
                    && !grid[i + 1][j].chars().collect::<Vec<char>>()[0].is_numeric()
                {
                    engine = true;
                } else if i > 0
                    && grid[i - 1][j] != "."
                    && !grid[i - 1][j].chars().collect::<Vec<char>>()[0].is_numeric()
                {
                    engine = true;
                }
            } else {
                if processing_number {
                    decimal = 0;

                    if grid[i][j] != "."
                        && !grid[i][j].chars().collect::<Vec<char>>()[0].is_numeric()
                    {
                        engine = true;
                    } else if i < grid.len() - 1
                        && grid[i + 1][j] != "."
                        && !grid[i + 1][j].chars().collect::<Vec<char>>()[0].is_numeric()
                    {
                        engine = true;
                    } else if i > 0
                        && grid[i - 1][j] != "."
                        && !grid[i - 1][j].chars().collect::<Vec<char>>()[0].is_numeric()
                    {
                        engine = true;
                    }
                    if engine {
                        total += current_number;
                        engine = false;
                    }
                    processing_number = false;
                    current_number = 0;
                }
            }
        }
    }

    total
}

#[test]
fn test_day03_2() {
    let data = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(day03_2(data), 467835);
}

pub fn day03_2(data: &str) -> u32 {
    let grid: Vec<Vec<char>> = data
        .lines()
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .iter()
                .rev()
                .map(|x| *x)
                .collect::<Vec<char>>()
        })
        .collect();

    let mut processing_number = false;
    let mut current_number: u32 = 0;
    let mut decimal: u32 = 0;
    let mut engine = None;
    let mut gears = HashMap::new();
    let mut occurences = HashMap::new();
    let mut occurence = 0;
    let mut total = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col.is_numeric() {
                if processing_number {
                    decimal += 1;
                    current_number += col.to_digit(10).unwrap() * ((10 as u32).pow(decimal));
                } else if !processing_number {
                    current_number = col.to_digit(10).unwrap();
                    if j > 0 && grid[i][j - 1] == '*' {
                        engine = Some((i, j - 1));
                    } else if j > 0 && i < grid.len() - 1 && grid[i + 1][j - 1] == '*' {
                        engine = Some((i + 1, j - 1));
                    } else if j > 0 && i > 0 && grid[i - 1][j - 1] == '*' {
                        engine = Some((i - 1, j - 1));
                    }
                    processing_number = true;
                }
                if i < grid.len() - 1 && grid[i + 1][j] == '*' {
                    engine = Some((i + 1, j));
                } else if i > 0 && grid[i - 1][j] == '*' {
                    engine = Some((i - 1, j));
                }
            } else {
                if processing_number {
                    decimal = 0;

                    if grid[i][j] == '*' {
                        engine = Some((i, j));
                    } else if i < grid.len() - 1 && grid[i + 1][j] == '*' {
                        engine = Some((i + 1, j));
                    } else if i > 0 && grid[i - 1][j] == '*' {
                        engine = Some((i - 1, j));
                    }
                    if engine.is_some() {
                        if let Some(x) = gears.get(&engine.unwrap()) {
                            current_number *= x
                        }
                        if let Some(y) = occurences.get(&engine.unwrap()) {
                            occurence += y;
                        }

                        gears.insert(engine.unwrap(), current_number);
                        occurences.insert(engine.unwrap(), occurence + 1);
                        engine = None;
                        occurence = 0;
                    }
                    processing_number = false;
                    current_number = 0;
                }
            }
        }
    }
    for key in gears.keys() {
        if let Some(x) = occurences.get(&key) {
            if *x == 2 {
                total += gears.get(&key).unwrap()
            }
        }
    }

    total
}
