/// Advent of Code 2019, day 2
/// https://adventofcode.com/2019/day/2

fn parse_input() -> Vec<usize> {
    let data = include_bytes!("input/d02.txt");
    String::from_utf8_lossy(data)
        .split(',')
        .map(|i| i.parse::<usize>().unwrap())
        .collect()
}

fn run_program(initial: Vec<usize>, noun: usize, verb: usize) -> Vec<usize> {
    let mut state = initial.clone();
    state[1] = noun;
    state[2] = verb;
    let mut cursor = 0;
    loop {
        match state[cursor] {
            1 => {
                // Adds values in 1st and 2nd indices and assign to 3rd index
                let o = state[cursor + 3];
                state[o] = state[state[cursor + 1]] + state[state[cursor + 2]];
                cursor += 4;
            }
            2 => {
                // Multiples values in 1st and 2nd indices and assign to 3rd index
                let o = state[cursor + 3];
                state[o] = state[state[cursor + 1]] * state[state[cursor + 2]];
                cursor += 4;
            }
            99 => {
                // Halts program
                break;
            }
            x => panic!("Encountered invalid opcode: '{}'", x),
        };
    }

    state
}

pub fn d02a() -> String {
    run_program(parse_input(), 12, 2)[0].to_string()
}

pub fn d02b() -> String {
    let (noun, verb) = (0usize..100)
        .flat_map(|j| (0usize..100).map(move |i| (i, j)))
        .find(|(n, v)| run_program(parse_input(), *n, *v)[0] == 19690720)
        .unwrap();

    (100 * noun + verb).to_string()
}
