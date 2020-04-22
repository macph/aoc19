/// Advent of Code 2019, day 5
/// https://adventofcode.com/2019/day/5
use std::iter::once;
use std::str::from_utf8;

use super::intcode::Intcode;

fn setup_intcode() -> Intcode {
    let data = include_bytes!("input/d05.txt");
    from_utf8(data).unwrap().parse::<Intcode>().unwrap()
}

fn run_intcode(value: i64) -> i64 {
    let mut program = setup_intcode();
    program.run(once(value));
    let output = program.collect::<Vec<i64>>();
    output[output.len() - 1]
}

pub fn part_a() -> String {
    run_intcode(1).to_string()
}

pub fn part_b() -> String {
    run_intcode(5).to_string()
}
