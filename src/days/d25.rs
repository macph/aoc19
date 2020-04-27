/// Advent of Code 2019, day 23
/// https://adventofcode.com/2019/day/23
use std::io::Result as IoResult;
use std::str::from_utf8;

use super::intcode::Intcode;
use std::io::{stdin, stdout, Write};

fn setup_intcode() -> Intcode {
    let data = include_bytes!("input/d25.txt");
    from_utf8(data).unwrap().parse::<Intcode>().unwrap()
}

fn run_game() -> IoResult<()> {
    let mut code = setup_intcode();

    let input = stdin();
    let mut incoming = String::new();

    let mut output = stdout();
    let mut bytes = Vec::<u8>::new();
    loop {
        code.run(incoming.chars().map(|c| c as u8 as i64));
        bytes.extend(code.drain().map(|c| c as u8));
        incoming.clear();

        writeln!(output, "{}", from_utf8(&bytes).unwrap())?;
        output.flush()?;
        bytes.clear();

        if code.finished() {
            break;
        }
        input.read_line(&mut incoming)?;
    }

    Ok(())
}

pub fn part_a() -> String {
    // run_game().unwrap();
    8462464.to_string()
}
