#[test]
fn test_day02_1() {
    let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(day02_1(data), 8);
}

pub fn day02_1(data: &str) -> u32 {
    data.lines()
        .map(|x| parse_line(x))
        .map(|x| match x {
            Some(y) => y,
            None => 0,
        })
        .sum()
}

fn parse_line(line: &str) -> Option<u32> {
    let parts: Vec<&str> = line.split(":").collect();
    let id = parts[0]
        .replace("Game ", "")
        .parse::<u32>()
        .expect("Not a number");
    let trials: Vec<Vec<Vec<&str>>> = parts[1]
        .split("; ")
        .map(|x| x.split(", ").map(|y| y.split(" ").collect()).collect())
        .collect();
    for trial in trials {
        for cube_type in trial {
            if cube_type[1] == "blue" && cube_type[0].parse::<u32>().expect("Not a number") > 14 {
                return None;
            }
            if cube_type[1] == "green" && cube_type[0].parse::<u32>().expect("Not a number") > 13 {
                return None;
            }
            if cube_type[1] == "red" && cube_type[0].parse::<u32>().expect("Not a number") > 12 {
                return None;
            }
        }
    }
    Some(id)
}

#[test]
fn test_day02_2() {
    let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(day02_2(data), 2286);
}

fn find_power(line: &str) -> u32 {
    let trials: Vec<Vec<Vec<&str>>> = line.split(": ").collect::<Vec<&str>>()[1]
        .split("; ")
        .map(|x| x.split(", ").map(|y| y.split(" ").collect()).collect())
        .collect();
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for trial in trials {
        for cube_type in trial {
            if cube_type[1] == "blue"
                && cube_type[0].parse::<u32>().expect("Not a number") > max_blue
            {
                max_blue = cube_type[0].parse::<u32>().expect("Not a number");
            }
            if cube_type[1] == "green"
                && cube_type[0].parse::<u32>().expect("Not a number") > max_green
            {
                max_green = cube_type[0].parse::<u32>().expect("Not a number");
            }
            if cube_type[1] == "red" && cube_type[0].parse::<u32>().expect("Not a number") > max_red
            {
                max_red = cube_type[0].parse::<u32>().expect("Not a number");
            }
        }
    }

    max_red * max_green * max_blue
}

pub fn day02_2(data: &str) -> u32 {
    data.lines().map(|x| find_power(x)).sum()
}
