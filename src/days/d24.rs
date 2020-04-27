/// Advent of Code 2019, day 24
/// https://adventofcode.com/2019/day/24
use std::collections::HashSet;
use std::str::{from_utf8, FromStr};

const WIDTH: u32 = 5;
const END: u32 = WIDTH - 1;
const HALF: u32 = WIDTH / 2;
const SIZE: u32 = WIDTH * WIDTH;
const CENTRE: u32 = SIZE / 2;

const fn as_coords(i: u32) -> (u32, u32) {
    (i % WIDTH, i / WIDTH)
}

const fn as_index(x: u32, y: u32) -> u32 {
    x + y * WIDTH
}

fn read_grid() -> &'static str {
    let data = include_bytes!("input/d24.txt");
    from_utf8(data).unwrap()
}

fn parse_grid(s: &str) -> u32 {
    let cells = s
        .chars()
        .filter_map(|c| match c {
            '.' | '?' => Some(0u32),
            '#' => Some(1u32),
            _ => None,
        })
        .enumerate()
        .fold(0u32, |a, (i, b)| a | b << i as u32);
    assert!(cells < (1 << SIZE));
    cells
}

#[derive(Debug, Copy, Clone)]
struct Grid(u32);

impl Grid {
    fn as_u32(&self) -> u32 {
        self.0
    }

    fn at_coords(&self, x: u32, y: u32) -> bool {
        (self.0 & 1 << as_index(x, y)) > 0
    }

    fn adjacent(&self, index: u32) -> u32 {
        let (x, y) = as_coords(index);
        [
            x > 0 && self.at_coords(x - 1, y),
            x < END && self.at_coords(x + 1, y),
            y > 0 && self.at_coords(x, y - 1),
            y < END && self.at_coords(x, y + 1),
        ]
        .iter()
        .filter(|&&a| a)
        .count() as u32
    }

    fn update(&mut self) {
        self.0 = (0..SIZE)
            .map(|i| {
                let shift = 1 << i;
                match (self.0 & shift, self.adjacent(i)) {
                    (0, 1) | (0, 2) => shift,
                    (0, _) => 0,
                    (_, 1) => shift,
                    (_, _) => 0,
                }
            })
            .fold(0u32, |a, b| a | b);
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(parse_grid(s)))
    }
}

fn run_grid() -> u32 {
    let mut v;
    let mut grid = read_grid().parse::<Grid>().unwrap();
    let mut set = HashSet::<u32>::new();
    loop {
        v = grid.as_u32();
        if !set.insert(v) {
            break v;
        }
        grid.update();
    }
}

const LEFT: u32 = as_index(HALF - 1, HALF);
const ABOVE: u32 = as_index(HALF, HALF - 1);
const RIGHT: u32 = as_index(HALF + 1, HALF);
const BELOW: u32 = as_index(HALF, HALF + 1);

#[derive(Debug, Clone)]
struct RecursiveGrid(Vec<u32>);

impl RecursiveGrid {
    fn at_index(&self, level: usize, index: u32) -> bool {
        let grid = *self.0.get(level).unwrap_or(&0);
        index != CENTRE && (grid & 1 << index) > 0
    }

    fn at_coords(&self, level: usize, x: u32, y: u32) -> bool {
        self.at_index(level, as_index(x, y))
    }

    fn deepest(&self) -> usize {
        self.0.len() - 1
    }

    fn adjacent(&self, level: usize, index: u32) -> u32 {
        let (x, y) = as_coords(index);
        let deep = self.deepest();
        let mut adj = 0;

        adj += (x > 0 && self.at_coords(level, x - 1, y)) as u32;
        adj += (y > 0 && self.at_coords(level, x, y - 1)) as u32;
        adj += (x < END && self.at_coords(level, x + 1, y)) as u32;
        adj += (y < END && self.at_coords(level, x, y + 1)) as u32;

        if level > 0 {
            adj += (x == 0 && self.at_coords(level - 1, HALF - 1, HALF)) as u32;
            adj += (y == 0 && self.at_coords(level - 1, HALF, HALF - 1)) as u32;
            adj += (x == END && self.at_coords(level - 1, HALF + 1, HALF)) as u32;
            adj += (y == END && self.at_coords(level - 1, HALF, HALF + 1)) as u32;
        }
        if level < deep && index == LEFT {
            adj += (0..WIDTH)
                .filter(|&y| self.at_coords(level + 1, 0, y))
                .count() as u32;
        }
        if level < deep && index == ABOVE {
            adj += (0..WIDTH)
                .filter(|&x| self.at_coords(level + 1, x, 0))
                .count() as u32;
        }
        if level < deep && index == RIGHT {
            adj += (0..WIDTH)
                .filter(|&y| self.at_coords(level + 1, END, y))
                .count() as u32;
        }
        if level < deep && index == BELOW {
            adj += (0..WIDTH)
                .filter(|&x| self.at_coords(level + 1, x, END))
                .count() as u32;
        }

        adj
    }

    fn update_level(&self, level: usize) -> u32 {
        let grid = *self.0.get(level).unwrap_or(&0);
        (0..SIZE)
            .map(|i| {
                if i == CENTRE {
                    return 0;
                }
                let shift = 1 << i;
                match (grid & shift, self.adjacent(level, i)) {
                    (0, 1) | (0, 2) => shift,
                    (0, _) => 0,
                    (_, 1) => shift,
                    (_, _) => 0,
                }
            })
            .fold(0u32, |a, b| a | b)
    }

    fn expand_up(&self) -> bool {
        (0..WIDTH).any(|i| {
            self.at_index(0, i)
                || self.at_index(0, i * WIDTH)
                || self.at_index(0, (SIZE - WIDTH) + i)
                || self.at_index(0, END + i * WIDTH)
        })
    }

    fn expand_down(&self) -> bool {
        let deep = self.deepest();
        self.at_index(deep, LEFT)
            || self.at_index(deep, ABOVE)
            || self.at_index(deep, RIGHT)
            || self.at_index(deep, BELOW)
    }

    fn update(&mut self) {
        if self.expand_up() {
            self.0.insert(0, 0);
        }
        if self.expand_down() {
            self.0.push(0);
        }
        self.0 = (0..self.0.len())
            .map(|level| self.update_level(level))
            .collect::<Vec<u32>>();
    }

    fn count(&self) -> usize {
        self.0
            .iter()
            .map(|g| (0u32..32).filter(|&i| g & 1 << i > 0).count())
            .sum()
    }
}

impl FromStr for RecursiveGrid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(vec![parse_grid(s)]))
    }
}

fn run_recursive_grid() -> usize {
    let mut grid = read_grid().parse::<RecursiveGrid>().unwrap();
    for _ in 0..200 {
        grid.update();
    }
    grid.count()
}

pub fn part_a() -> String {
    run_grid().to_string()
}

pub fn part_b() -> String {
    run_recursive_grid().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_recursive_grid() {
        let input = "\
            ....#\n\
            #..#.\n\
            #.?##\n\
            ..#..\n\
            #....";
        let mut grid = input.parse::<RecursiveGrid>().unwrap();
        assert_eq!(grid.0[0], 0b0000100100110010100110000);

        for _ in 0..10 {
            grid.update();
        }

        assert_eq!(
            grid.0,
            vec![
                0b0010001010100000101000100,
                0b0100011000000001100001000,
                0b0010100010000000001000101,
                0b0111011000100001000011010,
                0b1111001000000001100011001,
                0b0000000000000101101000010,
                0b1111111011100001100100110,
                0b0010111010000010101100111,
                0b1000100001000010000011100,
                0b0000001011000010100101110,
                0b0000001111010010100101111,
            ]
        );
    }
}
