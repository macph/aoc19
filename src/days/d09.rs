/// Advent of Code 2019, day 8
/// https://adventofcode.com/2019/day/8
use super::intcode::Intcode;
use std::iter::once;

fn setup_intcode() -> Intcode {
    let data = include_bytes!("input/d09.txt");
    Intcode::from_string(String::from_utf8_lossy(data).to_string())
}

pub fn d09a() -> String {
    let mut program = setup_intcode();
    program.run(once(1));
    program.nth(0).unwrap().to_string()
}

pub fn d09b() -> String {
    let mut program = setup_intcode();
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
        let data = parse("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let mut program = Intcode::from_vec(&data);
        program.run(empty());

        assert_eq!(program.collect::<Vec<i64>>(), data);
    }

    #[test]
    fn test_program_2() {
        let data = parse("1102,34915192,34915192,7,4,7,99,0");
        let mut program = Intcode::from_vec(&data);
        program.run(empty());
        let value = program.nth(0).unwrap();

        assert!(value >= 10i64.pow(15) && value < 10i64.pow(16));
    }

    #[test]
    fn test_program_3() {
        let data = parse("104,1125899906842624,99");
        let mut program = Intcode::from_vec(&data);
        program.run(empty());

        assert_eq!(program.nth(0).unwrap(), data[1]);
    }
}
