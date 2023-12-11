// x1 = x0 + y0
// y1 = y0 + z0
// x2 = x1 + y1 = x1 + y0 + z0 = x0 + 2 y0 + z0
// z1 = z0 + w0
// y2 = y1 + z1 = y0 + 2 z0 + w0
// x3 = x2 + y2 = x0 + 3 y0 + 3 z0 + w0
// w1 = w0 + a0
// z2 = z1 + w1 = z0 + 2 w0 + a0
// y3 = y2 + z2 = y0 + 3 z0 + 3 w0 + a0
// x4 = x3 + y3 = x0 + 4 y0 + 6 z0 + 4 w0 + a0

use num::integer::binomial;

#[test]
fn test_day09_1() {
    let data = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    assert_eq!(day09_1(data), 114);
}

#[test]
fn test_build_step() {
    assert_eq!(build_step(2, &vec![3]), 6);
}

fn build_step(i: usize, params: &[i64]) -> i64 {
    if params.len() > 0 {
        params
            .iter()
            .enumerate()
            .map(|(j, z)| binomial(i as i64, j as i64) * z)
            .reduce(|acc, z| acc + z)
            .unwrap()
    } else {
        0
    }
}

fn get_diffs(seq: &[i64]) -> Vec<i64> {
    let mut diffs = Vec::new();
    for (i, x) in seq.iter().enumerate() {
        if i == 0 {
            diffs.push(*x);
        } else {
            let diff = x - build_step(i, &diffs);

            diffs.push(diff);
        }
    }
    diffs
}

pub fn day09_1(data: &str) -> i64 {
    let seqs: Vec<Vec<i64>> = data
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|x| x.parse::<i64>().expect("not a number!"))
                .collect()
        })
        .collect();
    seqs.iter()
        .map(|seq: &Vec<i64>| build_step(seq.len(), &get_diffs(&seq)))
        .reduce(|acc, x| acc + x)
        .unwrap()
}

#[test]
fn test_day09_2() {
    let data = "10 13 16 21 30 45";
    assert_eq!(day09_2(data), 5);
}

pub fn day09_2(data: &str) -> i64 {
    let seqs: Vec<Vec<i64>> = data
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|x| x.parse::<i64>().expect("not a number!"))
                .collect::<Vec<i64>>()
                .iter()
                .rev()
                .map(|x| *x)
                .collect::<Vec<i64>>()
        })
        .collect();
    seqs.iter()
        .map(|seq: &Vec<i64>| build_step(seq.len(), &get_diffs(&seq)))
        .reduce(|acc, x| acc + x)
        .unwrap()
}
