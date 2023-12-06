// T total time
// p pedal time
// (T-p) travel time
// p velocity
// p * (T-p) distance
// r record
// distance > record
// Tp - p^2 > r
// -p^2 + Tp - r > 0
// p > (T +- sqrt(T^2 - 4r))/ 2
// (T - sqrt(T^2 - 4r))/ 2 < p < (T + sqrt(T^2 - 4r))/ 2

fn quad(T: &f64, r: &f64) -> (f64, f64) {
    let constant = T;
    let variable = (T * T - 4.0 * r).sqrt();
    ((constant - variable) / 2.0, (constant + variable) / 2.0)
}

pub fn day06_1(data: &str) -> f64 {
    let input: Vec<Vec<f64>> = data
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|x| x.parse().expect("not a number"))
                .collect()
        })
        .collect();
    let params: Vec<(f64, f64)> = input[0]
        .iter()
        .zip(input[1].iter())
        .map(|(x, y)| (*x, *y))
        .collect();
    println!("{params:?}");
    let count: Vec<f64> = params
        .iter()
        .map(|(T, r)| quad(T, r))
        .map(|(low, high)| {
            println!("{low}, {high}");
            (high.ceil()) - 1.0 - f64::max(low.floor(), 0.0)
        })
        .collect();
    println!("{count:?}");
    count.iter().map(|x| *x).reduce(|acc, x| acc * x).unwrap()
}

#[test]
fn test_day06_1() {
    let data = "7 15 30
9 40 200";
    assert_eq!(day06_1(data), 288.0);
}

#[test]
fn test_day06_2() {
    let data = "7 15 30
9 40 200";
    assert_eq!(day06_2(data), 71503.0);
}

pub fn day06_2(data: &str) -> f64 {
    day06_1(&data.replace(" ", ""))
}