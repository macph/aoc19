/// The Intcode computer used by several problems
use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

enum Mode {
    IMMEDIATE,
    POSITION,
    RELATIVE,
}

#[derive(Debug, Clone)]
pub struct Intcode {
    state: Vec<i64>,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    base: i64,
    pointer: usize,
    finished: bool,
}

impl Intcode {
    pub fn new(state: Vec<i64>) -> Self {
        Self {
            state,
            input: VecDeque::new(),
            output: VecDeque::new(),
            base: 0,
            pointer: 0,
            finished: false,
        }
    }

    fn current(&self) -> i64 {
        self.state[self.pointer]
    }

    fn read_mode(&self, position: usize, index: usize) -> Mode {
        let modes = self.state[position] / 100;
        match (modes / 10_i64.pow(index as u32)) % 10 {
            0 => Mode::POSITION,
            1 => Mode::IMMEDIATE,
            2 => Mode::RELATIVE,
            m => panic!(
                "{} within opcode {} is not a valid parameter.",
                m, self.state[position]
            ),
        }
    }

    fn read(&self, position: usize, offset: usize) -> i64 {
        let value = self.state[position + offset];
        let rel_value = value + self.base;
        match self.read_mode(position, offset - 1) {
            Mode::IMMEDIATE => value,
            Mode::POSITION if value >= 0 && (value as usize) < self.state.len() => {
                self.state[value as usize]
            }
            Mode::POSITION if value >= 0 => 0,
            Mode::POSITION => panic!("Integer {} is to be used as index but is negative.", value),
            Mode::RELATIVE if rel_value >= 0 && (rel_value as usize) < self.state.len() => {
                self.state[rel_value as usize]
            }
            Mode::RELATIVE if rel_value >= 0 => 0,
            Mode::RELATIVE => panic!(
                "Integer {} + base {} is to be used as index but is negative.",
                value, self.base
            ),
        }
    }

    fn write(&mut self, position: usize, offset: usize, value: i64) {
        let index_s = match self.read_mode(position, offset - 1) {
            Mode::RELATIVE => self.state[position + offset] + self.base,
            _ => self.state[position + offset],
        };
        if index_s < 0 {
            panic!(
                "Integer {} plus offset {} is to be used as index but is negative.",
                self.state[position + offset],
                index_s - self.state[position + offset]
            );
        }
        let index = index_s as usize;
        if index >= self.state.len() {
            self.state.resize(index + 1, 0);
        }
        self.state[index] = value;
    }

    fn add(&mut self) {
        self.write(
            self.pointer,
            3,
            self.read(self.pointer, 1) + self.read(self.pointer, 2),
        );
        self.pointer += 4;
    }

    fn multiply(&mut self) {
        self.write(
            self.pointer,
            3,
            self.read(self.pointer, 1) * self.read(self.pointer, 2),
        );
        self.pointer += 4;
    }

    fn write_input(&mut self) {
        let v = self.input.pop_front().unwrap();
        self.write(self.pointer, 1, v);
        self.pointer += 2;
    }

    fn read_output(&mut self) {
        self.output.push_back(self.read(self.pointer, 1));
        self.pointer += 2;
    }

    fn jump_if_true(&mut self) {
        if self.read(self.pointer, 1) != 0 {
            self.pointer = self.read(self.pointer, 2) as usize;
        } else {
            self.pointer += 3;
        }
    }

    fn jump_if_false(&mut self) {
        if self.read(self.pointer, 1) == 0 {
            self.pointer = self.read(self.pointer, 2) as usize;
        } else {
            self.pointer += 3;
        }
    }

    fn less_than(&mut self) {
        self.write(
            self.pointer,
            3,
            if self.read(self.pointer, 1) < self.read(self.pointer, 2) {
                1
            } else {
                0
            },
        );
        self.pointer += 4;
    }

    fn equals(&mut self) {
        self.write(
            self.pointer,
            3,
            if self.read(self.pointer, 1) == self.read(self.pointer, 2) {
                1
            } else {
                0
            },
        );
        self.pointer += 4;
    }

    fn set_base(&mut self) {
        self.base += self.read(self.pointer, 1);
        self.pointer += 2;
    }

    pub fn set_value(&mut self, position: usize, value: i64) {
        self.state[position] = value;
    }

    pub fn drain(&mut self) -> impl Iterator<Item = i64> + '_ {
        self.output.drain(..)
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn run<I>(&mut self, input: I) -> bool
    where
        I: Iterator<Item = i64>,
    {
        for i in input {
            self.input.push_back(i);
        }

        loop {
            match self.current() % 100 {
                1 => self.add(),
                2 => self.multiply(),
                3 if self.input.len() == 0 => break,
                3 => self.write_input(),
                4 => self.read_output(),
                5 => self.jump_if_true(),
                6 => self.jump_if_false(),
                7 => self.less_than(),
                8 => self.equals(),
                9 => self.set_base(),
                99 => {
                    self.finished = true;
                    break;
                }
                c => panic!("Opcode '{}' in code '{}' is not valid.", c, self.current()),
            }
        }

        !self.finished
    }
}

impl Iterator for Intcode {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.output.pop_front()
    }
}

impl FromStr for Intcode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = s
            .split(',')
            .map(|i| i.parse::<i64>())
            .collect::<Result<Vec<i64>, ParseIntError>>()?;
        Ok(Self::new(state))
    }
}
