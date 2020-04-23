/// Advent of Code 2019, day 19
/// https://adventofcode.com/2019/day/19
use std::collections::VecDeque;
use std::str::from_utf8;

use super::intcode::Intcode;

fn setup_intcode() -> Intcode {
    let data = include_bytes!("input/d19.txt");
    from_utf8(data).unwrap().parse::<Intcode>().unwrap()
}

fn within_beam(code: &Intcode, x: i64, y: i64) -> bool {
    let mut cloned = code.clone();
    cloned.run([x, y].iter().cloned());
    cloned.next().unwrap() != 0
}

#[derive(Debug)]
enum Side {
    Left,
    Right,
}

#[derive(Debug)]
struct Beam {
    code: Intcode,
    side: Side,
    x: i64,
    y: i64,
}

impl Beam {
    fn new(code: Intcode, side: Side) -> Self {
        Self {
            code,
            side,
            x: 0,
            y: 0,
        }
    }

    fn within(&self, x: i64, y: i64) -> bool {
        within_beam(&self.code, x, y)
    }

    fn find_left(&mut self) {
        loop {
            self.y += 1;
            let found = (self.x..self.x + self.y * 2).find(|&x| self.within(x, self.y));
            if let Some(x) = found {
                self.x = x;
                break;
            }
        }
    }

    fn find_right(&mut self) {
        loop {
            self.y += 1;
            let found = (self.x..self.x + self.y * 2)
                .skip_while(|&x| !self.within(x, self.y))
                .find(|&x| !self.within(x, self.y));
            if let Some(x) = found {
                self.x = x - 1;
                break;
            }
        }
    }
}

impl Iterator for Beam {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let c = (self.x, self.y);
        match self.side {
            Side::Left => self.find_left(),
            Side::Right => self.find_right(),
        };
        Some(c)
    }
}

#[derive(Debug, Copy, Clone)]
struct Row {
    y: i64,
    x0: i64,
    x1: i64,
}

impl Row {
    fn new(y: i64, x0: i64, x1: i64) -> Self {
        Self { y, x0, x1 }
    }
}

pub fn part_a() -> String {
    let program = setup_intcode();
    let size: i64 = 50;
    (0..size)
        .flat_map(|y| (0..size).map(move |x| (x, y)))
        .filter(|&(x, y)| within_beam(&program, x, y))
        .count()
        .to_string()
}

pub fn part_b() -> String {
    let size = 100usize;
    let program = setup_intcode();
    let mut queue = VecDeque::<Row>::with_capacity(size);

    let left = Beam::new(program.clone(), Side::Left);
    let right = Beam::new(program.clone(), Side::Right);

    for (l, r) in left.zip(right) {
        assert_eq!(l.1, r.1);
        if queue.len() == size {
            queue.pop_back();
        }
        queue.push_front(Row::new(l.1, l.0, r.0));

        if queue.len() < size || r.0 - l.0 + 1 < size as i64 {
            continue;
        }

        let top = queue.back().unwrap();
        let bottom = queue.front().unwrap();
        assert_eq!(bottom.y - top.y + 1, size as i64);
        if top.x1 - bottom.x0 + 1 >= size as i64 {
            return (bottom.x0 * 10000 + top.y).to_string();
        }
    }
    unreachable!()
}
