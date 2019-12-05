/// Advent of Code 2019, day 5
/// https://adventofcode.com/2019/day/5

fn parse_input() -> Vec<i32> {
    let data = include_bytes!("input/d05.txt");
    String::from_utf8_lossy(data)
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

enum Mode {
    IMMEDIATE,
    POSITION,
}

struct State(Vec<i32>);

impl State {
    fn from_vec(initial: Vec<i32>) -> State {
        State(initial.clone())
    }

    fn get(&self, position: usize) -> i32 {
        self.0[position]
    }

    fn read(&self, position: usize, mode: Mode) -> i32 {
        match mode {
            Mode::IMMEDIATE => self.0[position],
            Mode::POSITION if self.0[position] >= 0 => self.0[self.0[position] as usize],
            Mode::POSITION => panic!(
                "Integer {} is to be used as index but is negative.",
                self.0[position]
            ),
        }
    }

    fn write(&mut self, position: usize, value: i32) {
        if self.0[position] >= 0 {
            let index = self.0[position] as usize;
            self.0[index] = value;
        } else {
            panic!(
                "Integer {} is to be used as index but is negative.",
                self.0[position]
            );
        }
    }
}

fn read_mode(modes: i32, index: u32) -> Mode {
    if (modes / 10_i32.pow(index)) % 10 > 0 {
        Mode::IMMEDIATE
    } else {
        Mode::POSITION
    }
}

fn run_program(initial: Vec<i32>, input: Vec<i32>) -> Vec<i32> {
    let mut state = State::from_vec(initial);
    let mut input = input.clone();
    let mut output: Vec<i32> = Vec::new();
    let mut pointer: usize = 0;
    loop {
        let op_code = state.get(pointer) % 100;
        let modes = state.get(pointer) / 100;
        match op_code {
            1 => {
                // Add values in positions 1 and 2 and write to position 3
                let a = state.read(pointer + 1, read_mode(modes, 0));
                let b = state.read(pointer + 2, read_mode(modes, 1));
                state.write(pointer + 3, a + b);
                pointer += 4;
            }
            2 => {
                // Multiply values in positions 1 and 2 and write to position 3
                let a = state.read(pointer + 1, read_mode(modes, 0));
                let b = state.read(pointer + 2, read_mode(modes, 1));
                state.write(pointer + 3, a * b);
                pointer += 4;
            }
            3 => {
                // Write value from input to position 1
                state.write(pointer + 1, input.pop().unwrap());
                pointer += 2;
            }
            4 => {
                // Get value from position 1 and write to output
                output.push(state.read(pointer + 1, read_mode(modes, 0)));
                pointer += 2;
            }
            5 => {
                // Move pointer to position 2 if value at position 1 is non-zero
                if state.read(pointer + 1, read_mode(modes, 0)) != 0 {
                    pointer = state.read(pointer + 2, read_mode(modes, 1)) as usize;
                } else {
                    pointer += 3;
                }
            }
            6 => {
                // Move pointer to position 2 if value at position 1 is zero
                if state.read(pointer + 1, read_mode(modes, 0)) == 0 {
                    pointer = state.read(pointer + 2, read_mode(modes, 1)) as usize;
                } else {
                    pointer += 3;
                }
            }
            7 => {
                // Write 1 to position 3 if value at position 1 is less than value at position 2,
                // else zero
                let a = state.read(pointer + 1, read_mode(modes, 0));
                let b = state.read(pointer + 2, read_mode(modes, 1));
                state.write(pointer + 3, if a < b { 1 } else { 0 });
                pointer += 4;
            }
            8 => {
                // Write 1 to position 3 if value at position 1 equals value at position 2, else
                // zero
                let a = state.read(pointer + 1, read_mode(modes, 0));
                let b = state.read(pointer + 2, read_mode(modes, 1));
                state.write(pointer + 3, if a == b { 1 } else { 0 });
                pointer += 4;
            }
            99 => {
                break;
            }
            x => panic!(
                "Opcode '{}' in code '{}' is not valid.",
                x,
                state.get(pointer)
            ),
        }
    }

    output
}

pub fn d05a() -> String {
    let output = run_program(parse_input(), vec![1]);
    output[output.len() - 1].to_string()
}

pub fn d05b() -> String {
    let output = run_program(parse_input(), vec![5]);
    output[output.len() - 1].to_string()
}
