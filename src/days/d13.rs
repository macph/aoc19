/// Advent of Code 2019, day 13
/// https://adventofcode.com/2019/day/13
use std::collections::HashMap;
use std::iter::{empty, once};
use std::str::from_utf8;

use super::intcode::Intcode;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn from(i: i64) -> Self {
        match i {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            x => panic!("Tile {} is not valid.", x),
        }
    }

    fn draw(&self) -> &str {
        match self {
            Tile::Empty => " ",
            Tile::Wall => "#",
            Tile::Block => "O",
            Tile::Paddle => "_",
            Tile::Ball => "@",
        }
    }
}

struct Screen {
    tiles: HashMap<(i64, i64), Tile>,
    score: i64,
}

impl Screen {
    fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            score: 0,
        }
    }

    fn score(&self) -> i64 {
        self.score
    }

    fn blocks(&self) -> usize {
        self.tiles.iter().filter(|(_, &t)| t == Tile::Block).count()
    }

    fn set(&mut self, x: i64, y: i64, t: i64) {
        if (x, y) != (-1, 0) {
            self.tiles.insert((x, y), Tile::from(t));
        } else {
            self.score = t;
        }
    }

    fn bottom(&self) -> i64 {
        self.tiles.keys().map(|&(_, y)| y).max().unwrap_or(0)
    }

    fn left(&self) -> i64 {
        self.tiles.keys().map(|&(x, _)| x).min().unwrap_or(0)
    }

    fn right(&self) -> i64 {
        self.tiles.keys().map(|&(x, _)| x).max().unwrap_or(0)
    }

    fn top(&self) -> i64 {
        self.tiles.keys().map(|&(_, y)| y).min().unwrap_or(0)
    }

    fn draw(&self) -> String {
        let mut output = String::new();
        for j in self.top()..=self.bottom() {
            for i in self.left()..=self.right() {
                output += self.tiles.get(&(i, j)).map(|t| t.draw()).unwrap_or(" ");
                if i != self.right() {
                    output += " ";
                }
            }
            output += "\n";
        }
        output += &self.score.to_string();
        output
    }
}

fn setup_intcode() -> Intcode {
    let data = include_bytes!("input/d13.txt");
    from_utf8(data).unwrap().parse::<Intcode>().unwrap()
}

pub fn d13a() -> String {
    let mut program = setup_intcode();
    let mut screen = Screen::new();
    program.run(empty());
    for o in program.collect::<Vec<i64>>().chunks(3) {
        screen.set(o[0], o[1], o[2]);
    }
    screen.blocks().to_string()
}

pub fn d13b() -> String {
    let mut program = setup_intcode();
    let mut screen = Screen::new();
    program.set_value(0, 2);
    program.run(empty());

    let mut ball: (i64, i64) = (0, 0);
    let mut paddle: (i64, i64) = (0, 0);
    loop {
        let input = if paddle.0 > ball.0 {
            -1
        } else if paddle.0 < ball.0 {
            1
        } else {
            0
        };
        program.run(once(input));

        let output: Vec<i64> = program.drain().collect();
        for o in output.chunks(3) {
            if (o[0], o[1]) != (-1, 0) {
                match Tile::from(o[2]) {
                    Tile::Ball => ball = (o[0], o[1]),
                    Tile::Paddle => paddle = (o[0], o[1]),
                    _ => (),
                }
            }
            screen.set(o[0], o[1], o[2]);
        }

        // println!("{}", screen.draw());
        if screen.blocks() == 0 {
            break;
        }
    }

    screen.score().to_string()
}
