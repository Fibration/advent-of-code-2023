use num::Integer;

fn get_step(step: (i32, i32), letter: char) -> (i32, i32) {
    if letter == '-' {
        if step == (0, 1) {
            return (0, 1);
        } else {
            return (0, -1);
        }
    }
    if letter == '|' {
        if step == (1, 0) {
            return (1, 0);
        } else {
            return (-1, 0);
        }
    }
    if letter == 'F' {
        if step == (-1, 0) {
            return (0, 1);
        } else {
            return (1, 0);
        }
    }
    if letter == 'J' {
        if step == (1, 0) {
            return (0, -1);
        } else {
            return (-1, 0);
        }
    }
    if letter == '7' {
        if step == (0, 1) {
            return (1, 0);
        } else {
            return (0, -1);
        }
    } else {
        // L
        if step == (1, 0) {
            return (0, 1);
        } else {
            return (-1, 0);
        }
    }
}

pub fn day10_1(init: (i32, i32), data: &str) -> u32 {
    let map: Vec<Vec<char>> = data.lines().map(|x| x.chars().collect()).collect();
    let mut start: (usize, usize) = (0, 0);
    for (i, x) in map.iter().enumerate() {
        for (j, y) in x.iter().enumerate() {
            if *y == 'S' {
                start = (i, j);
            }
        }
    }
    let mut done = false;
    let mut current = start;
    let mut step = init;
    let mut count = 0;
    while !done {
        count += 1;
        current = (
            (current.0 as i32 + step.0) as usize,
            (current.1 as i32 + step.1) as usize,
        );
        if map[current.0][current.1] == 'S' {
            done = true;
        } else {
            step = get_step(step, map[current.0][current.1]);
        }
    }
    count
}


// Not sure what is wrong
pub fn day10_2(init: (i32, i32), data: &str) -> usize {
    let mut map: Vec<Vec<char>> = data.lines().map(|x| x.chars().collect()).collect();
    let mut start: (usize, usize) = (0, 0);

    for (i, x) in map.iter().enumerate() {
        for (j, y) in x.iter().enumerate() {
            if *y == 'S' {
                start = (i, j);
            }
        }
    }
    let mut done = false;
    let mut current = start;
    let mut step = init;
    let mut up = true;

    while !done {
        current = (
            (current.0 as i32 + step.0) as usize,
            (current.1 as i32 + step.1) as usize,
        );

        if map[current.0][current.1] == 'S' {
            done = true;
        } else {
            step = get_step(step, map[current.0][current.1]);
        }
        if up && step.0 > 0 {
            up = false;
        }
        if !up && step.0 < 0 {
            up = true;
        }
        if up {
            map[current.0][current.1] = 'X';
        } else {
            map[current.0][current.1] = 'Y';
        }
    }

    let mut inside = Vec::new();
    for (i, x) in map.iter().enumerate() {
        for (j, y) in x.iter().enumerate() {
            if *y == 'X' || *y == 'Y' {
                continue;
            }

            let right = x[(j + 1)..].iter().filter(|z| **z == 'X').count() as i32
                - x[(j + 1)..].iter().filter(|z| **z == 'Y').count() as i32;
            let left = x[..j].iter().filter(|z| **z == 'X').count() as i32
                - x[..j].iter().filter(|z| **z == 'Y').count() as i32;
            let up = map[..i].iter().filter(|z| z[j] == 'X').count() as i32
                - map[..i].iter().filter(|z| z[j] == 'Y').count() as i32;
            let down = map[(i + 1)..].iter().filter(|z| z[j] == 'X').count() as i32
                - map[(i + 1)..].iter().filter(|z| z[j] == 'Y').count() as i32;
            if right != 0 && left != 0 && up != 0 && down != 0 {
                inside.push((i, j));
            }
        }
    }
    for coord in &inside {
        map[coord.0][coord.1] = ' ';
    }
    let output: String = map
        .iter()
        .map(|x| x.iter().collect::<String>())
        .map(|x| x + "\n")
        .collect();
    println!("{output}");
    inside.len()
}
