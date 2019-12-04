/// Advent of Code 2019, day 1
/// https://adventofcode.com/2019/day/1

fn parse_input() -> Vec<i32> {
    let data = include_bytes!("d01.txt");
    String::from_utf8_lossy(data)
        .split('\n')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

fn fuel_required(mass: i32) -> i32 {
    mass / 3 - 2
}

pub fn d01a() -> String {
    parse_input()
        .iter()
        .map(|&mass| fuel_required(mass))
        .sum::<i32>()
        .to_string()
}

pub fn d01b() -> String {
    parse_input()
        .iter()
        .map(|&mass| {
            let mut total = 0;
            let mut m = fuel_required(mass);
            while m > 0 {
                total += m;
                m = fuel_required(m);
            }
            total
        })
        .sum::<i32>()
        .to_string()
}
