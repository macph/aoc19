/// Advent of Code 2019, day 5
/// https://adventofcode.com/2019/day/5
use super::intcode::Intcode;
use std::iter::once;

fn setup_intcode() -> Intcode {
    let data = include_bytes!("input/d05.txt");
    Intcode::from_string(String::from_utf8_lossy(data).to_string())
}

fn run_intcode(value: i64) -> i64 {
    let mut program = setup_intcode();
    program.run(once(value));
    let output = program.collect::<Vec<i64>>();
    output[output.len() - 1]
}

pub fn d05a() -> String {
    run_intcode(1).to_string()
}

pub fn d05b() -> String {
    run_intcode(5).to_string()
}
