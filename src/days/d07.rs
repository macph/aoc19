/// Advent of Code 2019, day 5
/// https://adventofcode.com/2019/day/5
use itertools::Itertools;
use std::collections::VecDeque;

enum Mode {
    IMMEDIATE,
    POSITION,
}

#[derive(Debug)]
pub struct Intcode {
    state: Vec<i32>,
    input: VecDeque<i32>,
    output: VecDeque<i32>,
    pointer: usize,
    finished: bool,
}

impl Intcode {
    pub fn from_vec(initial: &Vec<i32>) -> Intcode {
        Intcode {
            state: initial.clone(),
            input: VecDeque::new(),
            output: VecDeque::new(),
            pointer: 0,
            finished: false,
        }
    }

    fn current(&self) -> i32 {
        self.state[self.pointer]
    }

    fn read_mode(&self, index: usize) -> Mode {
        let modes = self.state[self.pointer] / 100;
        if (modes / 10_i32.pow(index as u32)) % 10 > 0 {
            Mode::IMMEDIATE
        } else {
            Mode::POSITION
        }
    }

    fn read(&self, offset: usize) -> i32 {
        match self.read_mode(offset - 1) {
            Mode::IMMEDIATE => self.state[self.pointer + offset],
            Mode::POSITION if self.state[self.pointer] >= 0 => {
                self.state[self.state[self.pointer + offset] as usize]
            }
            Mode::POSITION => panic!(
                "Integer {} is to be used as index but is negative.",
                self.state[self.pointer]
            ),
        }
    }

    fn write(&mut self, offset: usize, value: i32) {
        if self.state[self.pointer + offset] >= 0 {
            let index = self.state[self.pointer + offset] as usize;
            self.state[index] = value;
        } else {
            panic!(
                "Integer {} is to be used as index but is negative.",
                self.state[self.pointer + offset]
            );
        }
    }

    fn add(&mut self) {
        self.write(3, self.read(1) + self.read(2));
        self.pointer += 4;
    }

    fn multiply(&mut self) {
        self.write(3, self.read(1) * self.read(2));
        self.pointer += 4;
    }

    fn write_input(&mut self) {
        let v = self.input.pop_front().unwrap();
        self.write(1, v);
        self.pointer += 2;
    }

    fn read_output(&mut self) {
        self.output.push_back(self.read(1));
        self.pointer += 2;
    }

    fn jump_if_true(&mut self) {
        if self.read(1) != 0 {
            self.pointer = self.read(2) as usize;
        } else {
            self.pointer += 3;
        }
    }

    fn jump_if_false(&mut self) {
        if self.read(1) == 0 {
            self.pointer = self.read(2) as usize;
        } else {
            self.pointer += 3;
        }
    }

    fn less_than(&mut self) {
        self.write(3, if self.read(1) < self.read(2) { 1 } else { 0 });
        self.pointer += 4;
    }

    fn equals(&mut self) {
        self.write(3, if self.read(1) == self.read(2) { 1 } else { 0 });
        self.pointer += 4;
    }

    pub fn pop_output(&mut self) -> Option<i32> {
        self.output.pop_front()
    }

    pub fn drain_output(&mut self) -> Vec<i32> {
        self.output.drain(..).collect()
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn run(&mut self, input: Vec<i32>) -> bool {
        for i in input {
            self.input.push_back(i);
        }

        loop {
            match self.current() % 100 {
                1 => self.add(),
                2 => self.multiply(),
                3 => {
                    if self.input.len() > 0 {
                        self.write_input()
                    } else {
                        break;
                    }
                }
                4 => self.read_output(),
                5 => self.jump_if_true(),
                6 => self.jump_if_false(),
                7 => self.less_than(),
                8 => self.equals(),
                99 => {
                    self.finished = true;
                    break;
                }
                x => panic!("Opcode '{}' in code '{}' is not valid.", x, self.current()),
            }
        }

        !self.finished
    }
}

fn parse_input() -> Vec<i32> {
    let data = include_bytes!("input/d07.txt");
    String::from_utf8_lossy(data)
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

fn run_amplifiers_once(data: &Vec<i32>, phase: Vec<i32>) -> i32 {
    let mut value: i32 = 0;
    for amp in 0usize..5 {
        let mut program = Intcode::from_vec(data);
        program.run(vec![phase[amp], value]);
        value = program.pop_output().unwrap();
    }
    value
}

pub fn d07a() -> String {
    let data = parse_input();
    (0..5)
        .permutations(5)
        .map(|p| run_amplifiers_once(&data, p))
        .max()
        .unwrap()
        .to_string()
}

fn run_amplifiers_loop(data: &Vec<i32>, phase: Vec<i32>) -> i32 {
    let mut amplifiers: Vec<Intcode> = (0..5)
        .map(|i| {
            let mut p = Intcode::from_vec(&data);
            p.run(vec![phase[i]]);
            p
        })
        .collect();
    let mut signals: Vec<i32> = vec![0];
    loop {
        for i in 0..5 {
            amplifiers[i].run(signals.clone());
            signals = amplifiers[i].drain_output();
        }
        if amplifiers[4].finished() {
            break;
        }
    }
    signals[0]
}

pub fn d07b() -> String {
    let data = parse_input();
    (5..10)
        .permutations(5)
        .map(|p| run_amplifiers_loop(&data, p))
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse(string: &str) -> Vec<i32> {
        string
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    }

    #[test]
    fn test_amplifiers_a1() {
        let data = parse("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        assert_eq!(run_amplifiers_once(&data, vec![4, 3, 2, 1, 0]), 43210);
    }

    #[test]
    fn test_amplifiers_a2() {
        let data =
            parse("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
        assert_eq!(run_amplifiers_once(&data, vec![0, 1, 2, 3, 4]), 54321);
    }

    #[test]
    fn test_amplifiers_a3() {
        let data = parse(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,\
            4,31,99,0,0,0"
        );
        assert_eq!(run_amplifiers_once(&data, vec![1, 0, 4, 3, 2]), 65210);
    }

    #[test]
    fn test_amplifiers_b1() {
        let data = parse(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        assert_eq!(run_amplifiers_loop(&data, vec![9, 8, 7, 6, 5]), 139629729);
    }

    #[test]
    fn test_amplifiers_b2() {
        let data = parse(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,\
             53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,\
             10",
        );
        assert_eq!(run_amplifiers_loop(&data, vec![9, 7, 8, 5, 6]), 18216);
    }
}
