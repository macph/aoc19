/// Advent of Code 2019, day 10
/// https://adventofcode.com/2019/day/10
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::{empty, once};

use super::intcode::Intcode;

type Square = (i32, i32);

enum Direction {
    DOWN,
    LEFT,
    RIGHT,
    UP,
}

impl Direction {
    fn turn(&self, clockwise: bool) -> Direction {
        match self {
            Direction::DOWN if clockwise => Direction::LEFT,
            Direction::DOWN => Direction::RIGHT,
            Direction::LEFT if clockwise => Direction::UP,
            Direction::LEFT => Direction::DOWN,
            Direction::RIGHT if clockwise => Direction::DOWN,
            Direction::RIGHT => Direction::UP,
            Direction::UP if clockwise => Direction::RIGHT,
            Direction::UP => Direction::LEFT,
        }
    }

    fn move_from(&self, position: Square) -> Square {
        match self {
            Direction::DOWN => (position.0, position.1 + 1),
            Direction::LEFT => (position.0 - 1, position.1),
            Direction::RIGHT => (position.0 + 1, position.1),
            Direction::UP => (position.0, position.1 - 1),
        }
    }
}

struct Panel {
    panel: HashSet<Square>,
    painted: HashSet<Square>,
}

impl Panel {
    pub fn new() -> Panel {
        Panel {
            panel: HashSet::new(),
            painted: HashSet::new(),
        }
    }

    pub fn get(&self, square: Square) -> bool {
        self.panel.contains(&square)
    }

    pub fn set(&mut self, position: Square, value: bool) {
        match value {
            false => {
                self.panel.remove(&position);
            }
            true => {
                self.panel.insert(position);
                self.painted.insert(position);
            }
        };
    }

    fn bottom(&self) -> Option<i32> {
        self.panel.iter().map(|&(_, y)| y).min()
    }

    fn left(&self) -> Option<i32> {
        self.panel.iter().map(|&(x, _)| x).min()
    }

    fn right(&self) -> Option<i32> {
        self.panel.iter().map(|&(x, _)| x).max()
    }

    fn top(&self) -> Option<i32> {
        self.panel.iter().map(|&(_, y)| y).max()
    }

    fn render(&self) -> Option<String> {
        let (b, l, r, t) = (self.bottom()?, self.left()?, self.right()?, self.top()?);
        let panel = (b..=t)
            .map(|j| {
                (l..=r)
                    .map(|i| {
                        if self.panel.contains(&(i, j)) {
                            "#"
                        } else {
                            " "
                        }
                    })
                    .join(" ")
            })
            .join("\n");
        Some(panel)
    }

    fn total_painted(&self) -> usize {
        self.painted.len()
    }
}

fn to_bool(v: i64) -> bool {
    match v {
        0 => false,
        1 => true,
        x => panic!("Expected 0 or 1, got {}", x),
    }
}

fn setup_intcode() -> Intcode {
    let data = include_bytes!("input/d11.txt");
    Intcode::from_string(String::from_utf8_lossy(data).to_string())
}

fn paint_hull(initial: impl Iterator<Item = Square>) -> Panel {
    let program = &mut setup_intcode();
    let mut panel = Panel::new();
    initial.for_each(|s| panel.set(s, true));

    let mut position: Square = (0, 0);
    let mut direction = Direction::UP;
    loop {
        program.run(once(panel.get(position) as i64));
        let output: Vec<i64> = program.collect();
        if output.len() == 2 {
            panel.set(position, to_bool(output[0]));
            direction = direction.turn(to_bool(output[1]));
            position = direction.move_from(position);
        } else if program.finished() {
            break;
        } else {
            panic!("Program not finished but got invalid output {:?}", output);
        }
    }
    panel
}

pub fn d11a() -> String {
    paint_hull(empty()).total_painted().to_string()
}

pub fn d11b() -> String {
    paint_hull(once((0, 0))).render().unwrap()
}
