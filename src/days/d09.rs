/// Advent of Code 2019, day 9
/// https://adventofcode.com/2019/day/9
use std::iter::once;
use std::str::from_utf8;

use super::intcode::Intcode;

fn get_data<'a>() -> &'a str {
    let data = include_bytes!("input/d09.txt");
    from_utf8(data).unwrap()
}

pub fn d09a() -> String {
    let mut program = get_data().parse::<Intcode>().unwrap();
    program.run(once(1));
    program.nth(0).unwrap().to_string()
}

pub fn d09b() -> String {
    let mut program = get_data().parse::<Intcode>().unwrap();
    program.run(once(2));
    program.nth(0).unwrap().to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::empty;

    fn parse(string: &str) -> Vec<i64> {
        string
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }

    #[test]
    fn test_program_1() {
        let data = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut program = data.parse::<Intcode>().unwrap();
        program.run(empty());

        assert_eq!(program.collect::<Vec<i64>>(), parse(data));
    }

    #[test]
    fn test_program_2() {
        let data = "1102,34915192,34915192,7,4,7,99,0";
        let mut program = data.parse::<Intcode>().unwrap();
        program.run(empty());
        let value = program.nth(0).unwrap();

        assert!(value >= 10i64.pow(15) && value < 10i64.pow(16));
    }

    #[test]
    fn test_program_3() {
        let data = "104,1125899906842624,99";
        let mut program = data.parse::<Intcode>().unwrap();
        program.run(empty());

        assert_eq!(program.nth(0).unwrap(), parse(data)[1]);
    }
}
