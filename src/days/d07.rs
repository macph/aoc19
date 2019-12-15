/// Advent of Code 2019, day 7
/// https://adventofcode.com/2019/day/7
use std::iter::once;
use std::str::from_utf8;

use itertools::Itertools;

use super::intcode::Intcode;

fn get_data<'a>() -> &'a str {
    let data = include_bytes!("input/d07.txt");
    from_utf8(data).unwrap()
}

fn start_amplifier(data: &str, phase: i64) -> Intcode {
    let mut program = data.parse::<Intcode>().unwrap();
    program.run(once(phase));
    program
}

fn run_amplifiers(data: &str, phase: Vec<i64>) -> i64 {
    let mut amplifiers: Vec<Intcode> = (0..5).map(|i| start_amplifier(data, phase[i])).collect();
    let mut signals: Vec<i64> = vec![0];
    loop {
        for i in 0..5 {
            amplifiers[i].run(signals.iter().map(|&s| s));
            signals = amplifiers[i].drain().collect();
        }
        if amplifiers[4].finished() {
            break;
        }
    }
    signals[0]
}

pub fn d07a() -> String {
    let data = get_data();
    (0..5)
        .permutations(5)
        .map(|p| run_amplifiers(data, p))
        .max()
        .unwrap()
        .to_string()
}

pub fn d07b() -> String {
    let data = get_data();
    (5..10)
        .permutations(5)
        .map(|p| run_amplifiers(data, p))
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_amplifiers_a1() {
        let data = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        assert_eq!(run_amplifiers(data, vec![4, 3, 2, 1, 0]), 43210);
    }

    #[test]
    fn test_amplifiers_a2() {
        let data = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        assert_eq!(run_amplifiers(data, vec![0, 1, 2, 3, 4]), 54321);
    }

    #[test]
    fn test_amplifiers_a3() {
        let data =
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,\
            4,31,99,0,0,0";
        assert_eq!(run_amplifiers(data, vec![1, 0, 4, 3, 2]), 65210);
    }

    #[test]
    fn test_amplifiers_b1() {
        let data =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        assert_eq!(run_amplifiers(data, vec![9, 8, 7, 6, 5]), 139629729);
    }

    #[test]
    fn test_amplifiers_b2() {
        let data =
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,\
             53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,\
             10";
        assert_eq!(run_amplifiers(data, vec![9, 7, 8, 5, 6]), 18216);
    }
}
