/// Advent of Code 2019, day 17
/// https://adventofcode.com/2019/day/17
use std::collections::HashMap;
use std::iter::empty;
use std::slice::Iter;
use std::str::from_utf8;

use super::intcode::Intcode;

type C = (i32, i32);

#[derive(Debug, Clone)]
enum Direction {
    Unknown,
    Down,
    Left,
    Right,
    Up,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'X' => Direction::Unknown,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            x => panic!("Unknown character for direction: {}", x),
        }
    }

    fn iter() -> Iter<'static, Self> {
        static DIR: [Direction; 4] = [
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ];
        DIR.iter()
    }

    fn as_char(&self) -> char {
        match self {
            Direction::Unknown => 'X',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Up => '^',
        }
    }

    fn step(&self, position: C) -> C {
        match self {
            Direction::Unknown => position,
            Direction::Down => (position.0, position.1 + 1),
            Direction::Left => (position.0 - 1, position.1),
            Direction::Right => (position.0 + 1, position.1),
            Direction::Up => (position.0, position.1 - 1),
        }
    }
}

#[derive(Debug)]
struct Scaffold {
    map: HashMap<C, bool>,
    robot: Option<(C, Direction)>,
}

impl Scaffold {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            robot: None,
        }
    }

    fn from_output(output: impl Iterator<Item = i64>) -> Self {
        let mut s = Self::new();
        s.update(output);
        s
    }

    fn update(&mut self, output: impl Iterator<Item = i64>) {
        let mut newline;
        let mut position = (0, 0);
        for i in output {
            newline = false;
            match i as u8 as char {
                '#' => {
                    self.map.insert(position, true);
                }
                '.' => {
                    self.map.insert(position, false);
                }
                d @ 'v' | d @ '<' | d @ '>' | d @ '^' | d @ 'X' => {
                    self.map.insert(position, d != 'X');
                    self.robot = Some((position, Direction::from_char(d)));
                }
                '\n' => newline = true,
                c => panic!("Received unknown char {:?} ({}) in output", c, i),
            };
            if newline {
                position.0 = 0;
                position.1 += 1;
            } else {
                position.0 += 1;
            }
        }
    }

    fn adjacent(&self, c: C, d: Direction) -> bool {
        *self.map.get(&d.step(c)).unwrap_or(&false)
    }

    fn intersections(&self) -> Vec<C> {
        self.map
            .iter()
            .filter_map(|(&c, f)| {
                match *f && Direction::iter().all(|d| self.adjacent(c, d.clone())) {
                    true => Some(c),
                    false => None,
                }
            })
            .collect()
    }

    #[allow(dead_code)]
    fn draw(&self) {
        let bottom = self.map.keys().map(|c| c.1).max().unwrap();
        let left = self.map.keys().map(|c| c.0).min().unwrap();
        let right = self.map.keys().map(|c| c.0).max().unwrap();
        let top = self.map.keys().map(|c| c.1).min().unwrap();
        let robot = self.robot.as_ref();
        for j in top..=bottom {
            for i in left..=right {
                match robot {
                    Some(r) if r.0 == (i, j) => {
                        print!("{} ", r.1.as_char());
                        continue;
                    }
                    _ => match self.map.get(&(i, j)).unwrap_or(&false) {
                        true => print!("# "),
                        false => print!(". "),
                    },
                };
            }
            println!();
        }
    }
}

fn setup_intcode() -> Intcode {
    let data = include_bytes!("input/d17.txt");
    from_utf8(data).unwrap().parse::<Intcode>().unwrap()
}

pub fn part_a() -> String {
    let mut program = setup_intcode();
    program.run(empty());
    let scaffold = Scaffold::from_output(program.drain());
    // scaffold.draw();
    scaffold
        .intersections()
        .iter()
        .map(|&c| c.0 * c.1)
        .sum::<i32>()
        .to_string()
}

fn stream_chars(s: &str) -> impl Iterator<Item = i64> + '_ {
    s.chars().map(|c| c as u8 as i64)
}

pub fn part_b() -> String {
    let mut program = setup_intcode();

    program.set_value(0, 2);
    program.run(empty());
    program.run(stream_chars(
        "A,B,A,B,C,C,B,C,B,A\n\
         R,12,L,8,R,12\n\
         R,8,R,6,R,6,R,8\n\
         R,8,L,8,R,8,R,4,R,4\n\
         n\n",
    ));

    let output = program.drain().collect::<Vec<i64>>();
    output[output.len() - 1].to_string()
}
