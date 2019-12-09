/// The Intcode computer used by several problems
use std::collections::VecDeque;

enum Mode {
    IMMEDIATE,
    POSITION,
}

#[derive(Debug)]
pub struct Intcode {
    state: Vec<i64>,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    pointer: usize,
    finished: bool,
}

impl Intcode {
    pub fn from_vec(initial: &Vec<i64>) -> Intcode {
        Intcode {
            state: initial.clone(),
            input: VecDeque::new(),
            output: VecDeque::new(),
            pointer: 0,
            finished: false,
        }
    }

    pub fn from_string(string: String) -> Intcode {
        Intcode {
            state: string
                .split(',')
                .map(|i| i.parse::<i64>().unwrap())
                .collect(),
            input: VecDeque::new(),
            output: VecDeque::new(),
            pointer: 0,
            finished: false,
        }
    }

    fn current(&self) -> i64 {
        self.state[self.pointer]
    }

    fn read_mode(&self, position: usize, index: usize) -> Mode {
        let modes = self.state[position] / 100;
        if (modes / 10_i64.pow(index as u32)) % 10 > 0 {
            Mode::IMMEDIATE
        } else {
            Mode::POSITION
        }
    }

    fn read(&self, position: usize, offset: usize) -> i64 {
        match self.read_mode(position, offset - 1) {
            Mode::IMMEDIATE => self.state[position + offset],
            Mode::POSITION if self.state[position] >= 0 => {
                self.state[self.state[position + offset] as usize]
            }
            Mode::POSITION => panic!(
                "Integer {} is to be used as index but is negative.",
                self.state[position]
            ),
        }
    }

    fn write(&mut self, position: usize, value: i64) {
        if self.state[position] < 0 {
            panic!(
                "Integer {} is to be used as index but is negative.",
                self.state[position]
            );
        }
        let index = self.state[position] as usize;
        self.state[index] = value;
    }

    fn add(&mut self) {
        self.write(
            self.pointer + 3,
            self.read(self.pointer, 1) + self.read(self.pointer, 2),
        );
        self.pointer += 4;
    }

    fn multiply(&mut self) {
        self.write(
            self.pointer + 3,
            self.read(self.pointer, 1) * self.read(self.pointer, 2),
        );
        self.pointer += 4;
    }

    fn write_input(&mut self) {
        let v = self.input.pop_front().unwrap();
        self.write(self.pointer + 1, v);
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
            self.pointer + 3,
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
            self.pointer + 3,
            if self.read(self.pointer, 1) == self.read(self.pointer, 2) {
                1
            } else {
                0
            },
        );
        self.pointer += 4;
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
