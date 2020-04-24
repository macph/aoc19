/// Advent of Code 2019, day 21
/// https://adventofcode.com/2019/day/21
use std::fmt::{Display, Formatter, Result as FmtResult, Write};
use std::str::{from_utf8, FromStr};
use std::u8;

use super::intcode::Intcode;
use std::convert::TryInto;

fn setup_intcode() -> Intcode {
    let data = include_bytes!("input/d21.txt");
    from_utf8(data).unwrap().parse::<Intcode>().unwrap()
}

const REGISTERS: [char; 11] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'T'];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct R(char);

impl R {
    fn writeable(&self) -> bool {
        match self.0 {
            'J' | 'T' => true,
            _ => false,
        }
    }

    fn char(self) -> char {
        self.0
    }
}

impl FromStr for R {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let c = chars.next().ok_or(())?;
        if chars.next().is_some() || !REGISTERS.contains(&c) {
            Err(())?;
        }
        Ok(Self(c))
    }
}

#[derive(Debug, Copy, Clone)]
enum ParseInstructionError {
    Format,
    Keyword,
    Register,
    NotWriteable,
}

impl Display for ParseInstructionError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ParseInstructionError::Format => write!(f, "invalid format for instruction"),
            ParseInstructionError::Keyword => write!(f, "invalid keyword for instruction"),
            ParseInstructionError::Register => write!(f, "invalid register for instruction"),
            ParseInstructionError::NotWriteable => write!(f, "register not writeable"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    And(R, R),
    Or(R, R),
    Not(R, R),
    Walk,
    Run,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Instruction::And(l, r) => write!(f, "AND {} {}", l.char(), r.char()),
            Instruction::Or(l, r) => write!(f, "OR {} {}", l.char(), r.char()),
            Instruction::Not(l, r) => write!(f, "NOT {} {}", l.char(), r.char()),
            Instruction::Walk => write!(f, "WALK"),
            Instruction::Run => write!(f, "RUN"),
        }
    }
}

fn word(s: &str) -> Result<(&str, &str), ()> {
    let start = s.trim_start();
    match start.chars().position(|c| c.is_whitespace()) {
        Some(w) if w == 0 => Err(()),
        Some(w) => Ok((&start[w..].trim_start(), &start[..w])),
        None => Ok(("", start)),
    }
}

fn instruction(s: &str) -> Result<(&str, Instruction), ParseInstructionError> {
    let (mut rem, keyword) = word(s).or(Err(ParseInstructionError::Format))?;
    let next: usize = match keyword {
        "AND" | "OR" | "NOT" => 2,
        "WALK" | "RUN" => 0,
        _ => Err(ParseInstructionError::Keyword)?,
    };
    let mut reg = Vec::<R>::with_capacity(next);
    for _ in 0..next {
        let result = word(rem).or(Err(ParseInstructionError::Format))?;
        let word = result.1;
        reg.push(word.parse::<R>().or(Err(ParseInstructionError::Register))?);
        rem = result.0;
    }
    if ["AND", "OR", "NOT"].contains(&keyword) && !reg[1].writeable() {
        Err(ParseInstructionError::NotWriteable)?;
    }

    let ins = match keyword {
        "AND" => Instruction::And(reg[0], reg[1]),
        "OR" => Instruction::Or(reg[0], reg[1]),
        "NOT" => Instruction::Not(reg[0], reg[1]),
        "WALK" => Instruction::Walk,
        "RUN" => Instruction::Run,
        _ => unreachable!(),
    };
    Ok((rem, ins))
}

fn parse_instructions(s: &str) -> Result<Vec<Instruction>, ParseInstructionError> {
    let mut rem = s;
    let mut ins = Vec::new();
    while !rem.is_empty() {
        let result = instruction(rem)?;
        ins.push(result.1);
        rem = result.0;
    }
    Ok(ins)
}

fn execute_instructions(code: &mut Intcode, instructions: &[Instruction]) -> Option<i64> {
    let mut string = String::new();
    for i in instructions {
        write!(string, "{}\n", i).unwrap();
    }
    code.run(string.bytes().map(|b| b as i64));

    let mut bytes = Vec::<u8>::new();
    let mut result = Vec::<i64>::new();
    for i in code {
        match i.try_into() {
            Ok(b) => bytes.push(b),
            Err(_) => result.push(i),
        }
    }
    if !result.is_empty() {
        Some(result[0])
    } else {
        eprintln!("{}", from_utf8(&bytes).unwrap());
        None
    }
}

macro_rules! execute {
    ($($t:tt)*) => {{
        let parsed = parse_instructions(stringify!($($t)*)).unwrap();
        let mut program = setup_intcode();
        execute_instructions(&mut program, &parsed).unwrap()
    }}
}

pub fn part_a() -> String {
    let result = execute! {
        NOT J T // set T to true
        AND A T
        AND B T
        AND C T // check if any of next 3 squares are false
        NOT T J
        AND D J // to jump 4th square must be true and any of 1-3 squares false
        WALK
    };
    result.to_string()
}

pub fn part_b() -> String {
    let result = execute! {
        NOT J T
        AND A T
        AND B T
        AND C T
        NOT T J
        AND D J // check next 4 squares as above
        NOT E T
        NOT T T // copy 5th square to T
        OR  H T // if 5th square is false the 8th square must be true in order to jump again
        AND T J
        RUN
    };
    result.to_string()
}
