use std::fs;

fn d1_input() -> Vec<i32> {
    fs::read_to_string("input/d1.txt")
        .expect("'d1.txt' does not exist.")
        .split('\n')
        .filter_map(|s| match s.parse::<i32>() {
            Ok(m) => Some(fuel_required(m)),
            Err(_) => None
        })
        .collect()
}

fn fuel_required(mass: i32) -> i32 {
    mass / 3 - 2
}

fn d1a() -> String {
    d1_input().iter().sum::<i32>().to_string()
}

fn d1b() -> String {
    d1_input().iter()
        .map(|&mass| {
            let mut m = mass;
            let mut total = mass;
            loop {
                m = fuel_required(m);
                if m > 0 {
                    total += m;
                } else {
                    break;
                }
            }
            total
        })
        .sum::<i32>()
        .to_string()
}

fn main() {
    println!("Day 1: {}, {}", d1a(), d1b());
}
