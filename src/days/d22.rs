/// Advent of Code 2019, day 22
/// https://adventofcode.com/2019/day/22
use std::str::{from_utf8, FromStr};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Stack,
    Cut(i64),
    Deal(u64),
}

fn cut(size: u64, position: u64, cut: i64) -> u64 {
    if cut > 0 {
        let diff = cut as u64;
        (position + size - diff) % size
    } else if cut < 0 {
        let diff = -cut as u64;
        (position + size + diff) % size
    } else {
        position
    }
}

impl Instruction {
    fn execute(&self, size: u64, position: u64) -> u64 {
        match *self {
            Instruction::Stack => size - 1 - position,
            Instruction::Cut(n) => cut(size, position, n),
            Instruction::Deal(n) => (n * position) % size,
        }
    }

    fn reverse(&self, size: u64, position: u64) -> u64 {
        match *self {
            Instruction::Stack => size - 1 - position,
            Instruction::Cut(n) => cut(size, position, -n),
            Instruction::Deal(n) => {
                let mut before_mod = position;
                while before_mod % n != 0 {
                    before_mod += size;
                }
                before_mod / n
            },
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref STACK: Regex = Regex::new("deal into new stack").unwrap();
            static ref INCREMENT: Regex = Regex::new(r"deal with increment (\d+)").unwrap();
            static ref CUT: Regex = Regex::new(r"cut (-?\d+)").unwrap();
        }
        let i = if STACK.is_match(s) {
            Instruction::Stack
        } else if CUT.is_match(s) {
            let captures = CUT.captures(s).unwrap();
            let n = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
            Instruction::Cut(n)
        } else if INCREMENT.is_match(s) {
            let captures = INCREMENT.captures(s).unwrap();
            let n = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
            Instruction::Deal(n)
        } else {
            panic!("instruction {:?} not recognised", s);
        };
        Ok(i)
    }
}

fn read_instructions() -> Vec<Instruction> {
    let data = include_bytes!("input/d22.txt");
    from_utf8(data)
        .unwrap()
        .lines()
        .map(|line| line.parse::<Instruction>())
        .collect::<Result<Vec<Instruction>, ()>>()
        .unwrap()
}

pub fn part_a() -> String {
    let instructions = read_instructions();
    let size = 10007;
    let mut card = 2019;
    for i in instructions {
        card = i.execute(size, card);
    }
    card.to_string()
}

pub fn part_b() -> String {
    let instructions = read_instructions();
    let size = 119315717514047;
    let mut card = 2020;
    for _ in 0u64..101741582076661 {
        for i in instructions.iter().rev() {
            card = i.reverse(size, card);
        }
    }
    card.to_string()
}

#[test]
fn test_ten_cards() {
    let size = 10;
    let execute = |deck: &mut Vec<u64>, i: Instruction| {
        deck.iter_mut().for_each(|c| *c = i.execute(size, *c));
    };
    let reverse = |deck: &mut Vec<u64>, i: Instruction| {
        deck.iter_mut().for_each(|c| *c = i.reverse(size, *c));
    };
    let collect = |deck: &[u64]| {
        let mut new_deck = vec![0u64; 10];
        for (i, &c) in deck.iter().enumerate() {
            new_deck[c as usize] = i as u64;
        }
        new_deck
    };

    let mut deck = (0..size).collect::<Vec<u64>>();
    assert_eq!(collect(&deck), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    execute(&mut deck, Instruction::Stack);
    assert_eq!(collect(&deck), [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    execute(&mut deck, Instruction::Cut(-2));
    assert_eq!(collect(&deck), [1, 0, 9, 8, 7, 6, 5, 4, 3, 2]);
    execute(&mut deck, Instruction::Deal(7));
    assert_eq!(collect(&deck), [1, 8, 5, 2, 9, 6, 3, 0, 7, 4]);
    execute(&mut deck, Instruction::Cut(8));
    assert_eq!(collect(&deck), [7, 4, 1, 8, 5, 2, 9, 6, 3, 0]);

    reverse(&mut deck, Instruction::Cut(8));
    assert_eq!(collect(&deck), [1, 8, 5, 2, 9, 6, 3, 0, 7, 4]);
    reverse(&mut deck, Instruction::Deal(7));
    assert_eq!(collect(&deck), [1, 0, 9, 8, 7, 6, 5, 4, 3, 2]);
    reverse(&mut deck, Instruction::Cut(-2));
    assert_eq!(collect(&deck), [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    reverse(&mut deck, Instruction::Stack);
    assert_eq!(collect(&deck), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
