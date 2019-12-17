/// Advent of Code 2019, day 15
/// https://adventofcode.com/2019/day/15
use std::collections::{HashMap, VecDeque};
use std::iter::once;
use std::slice::Iter;
use std::str::from_utf8;

use super::intcode::Intcode;

fn setup_intcode() -> Intcode {
    let data = include_bytes!("input/d15.txt");
    from_utf8(data).unwrap().parse::<Intcode>().unwrap()
}

type Point = (i32, i32);

#[derive(Debug, Clone, Copy)]
enum Status {
    Empty,
    Wall,
    OxygenSystem,
}

impl Status {
    fn from_i64(i: i64) -> Self {
        match i {
            0 => Status::Wall,
            1 => Status::Empty,
            2 => Status::OxygenSystem,
            x => panic!("Output must be in range [0, 2], received {:?}", x),
        }
    }

    fn has_moved(&self) -> bool {
        match self {
            Status::Empty | Status::OxygenSystem => true,
            _ => false,
        }
    }

    fn is_oxygen_system(&self) -> bool {
        match self {
            Status::OxygenSystem => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn iter() -> Iter<'static, Self> {
        static DIR: [Direction; 4] = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];
        DIR.iter()
    }

    fn as_i64(&self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::East => 3,
            Direction::West => 4,
        }
    }

    fn move_from(&self, p: Point) -> Point {
        match self {
            Direction::North => (p.0, p.1 - 1),
            Direction::South => (p.0, p.1 + 1),
            Direction::East => (p.0 - 1, p.1),
            Direction::West => (p.0 + 1, p.1),
        }
    }
}

#[derive(Debug, Clone)]
struct Droid {
    position: Point,
    program: Intcode,
    travelled: u32,
}

impl Droid {
    fn new(code: &Intcode) -> Self {
        Self {
            position: (0, 0),
            program: code.clone(),
            travelled: 0,
        }
    }

    fn run(&mut self, d: Direction) -> Status {
        self.program.run(once(d.as_i64()));
        let output: Vec<i64> = self.program.drain().collect();
        if output.len() != 1 {
            panic!("Single output expected, received {:?}", output);
        }
        let status = Status::from_i64(output[0]);
        if status.has_moved() {
            self.position = d.move_from(self.position);
            self.travelled += 1;
        }
        status
    }

    fn reset(&mut self) {
        self.travelled = 0;
    }

    fn extend(&self, discovered: &mut HashMap<Point, Status>) -> Vec<Self> {
        let mut new_droids = Vec::new();
        for d in Direction::iter() {
            let next = d.move_from(self.position);
            if discovered.contains_key(&next) {
                continue;
            }
            let mut droid = self.clone();
            let status = droid.run(*d);
            if status.has_moved() {
                new_droids.push(droid);
            }
            discovered.insert(next, status);
        }
        new_droids
    }
}

fn find_oxygen_system(code: &Intcode) -> Option<Droid> {
    let mut discovered: HashMap<Point, Status> = HashMap::new();
    let mut queue: VecDeque<Droid> = VecDeque::new();
    queue.push_front(Droid::new(code));
    loop {
        let droid = queue.pop_back()?;
        for d in droid.extend(&mut discovered).drain(..) {
            if discovered.get(&d.position).unwrap().is_oxygen_system() {
                return Some(d);
            }
            queue.push_front(d);
        }
    }
}

pub fn d15a() -> String {
    let code = setup_intcode();
    find_oxygen_system(&code).unwrap().travelled.to_string()
}

pub fn d15b() -> String {
    let code = setup_intcode();
    let mut start = find_oxygen_system(&code).unwrap();
    start.reset();

    let mut discovered: HashMap<Point, Status> = HashMap::new();
    let mut queue: VecDeque<Droid> = VecDeque::new();
    queue.push_front(start);

    let mut greatest: u32 = 0;
    while let Some(droid) = queue.pop_back() {
        for d in droid.extend(&mut discovered).drain(..) {
            if d.travelled > greatest {
                greatest = d.travelled;
            }
            queue.push_front(d);
        }
    }

    greatest.to_string()
}
