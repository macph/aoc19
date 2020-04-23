/// Advent of Code 2019, day 20
/// https://adventofcode.com/2019/day/20
use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::{from_utf8, FromStr};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Portal(char, char);

const ENTRY: Portal = Portal('A', 'A');
const EXIT: Portal = Portal('Z', 'Z');

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Inward,
    Outward,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Feature {
    Empty,
    Wall,
    Path,
    Portal(Portal, Direction),
}

#[derive(Debug, Clone)]
struct Map {
    features: Vec<Feature>,
    width: usize,
    height: usize,
    portals: HashMap<Portal, (usize, usize)>,
    entry: usize,
    exit: usize,
}

impl Map {
    fn as_coords(&self, index: usize) -> (usize, usize) {
        if index < self.features.len() {
            (index % self.width, index / self.width)
        } else {
            panic!("index exceeds max length {}", self.features.len())
        }
    }

    fn as_index(&self, coords: (usize, usize)) -> usize {
        if coords.0 < self.width && coords.1 < self.height {
            coords.0 + self.width * coords.1
        } else {
            panic!(
                "coords exceed max width {} and height {}",
                self.width, self.height
            );
        }
    }

    fn check_adjacent(&self, index: usize, level: Option<usize>) -> bool {
        match self.features[index] {
            Feature::Empty | Feature::Wall => false,
            Feature::Path => true,
            Feature::Portal(ENTRY, _) => false,
            Feature::Portal(EXIT, _) => level.unwrap_or(0) == 0,
            Feature::Portal(_, Direction::Inward) => true,
            Feature::Portal(_, Direction::Outward) => level.unwrap_or(1) != 0,
        }
    }

    fn adjacent(&self, index: usize, level: Option<usize>) -> Vec<(usize, Option<usize>)> {
        let (x, y) = self.as_coords(index);
        let mut adj = Vec::<(usize, Option<usize>)>::with_capacity(4);

        let left = self.as_index((x - 1, y));
        if x > 0 && self.check_adjacent(left, level) {
            adj.push((left, level));
        }

        let right = self.as_index((x + 1, y));
        if x < self.width - 1 && self.check_adjacent(right, level) {
            adj.push((right, level));
        }

        let above = self.as_index((x, y - 1));
        if y > 0 && self.check_adjacent(above, level) {
            adj.push((above, level));
        }

        let below = self.as_index((x, y + 1));
        if y < self.height - 1 && self.check_adjacent(below, level) {
            adj.push((below, level));
        }

        match self.features[index] {
            Feature::Portal(p, d) if p != ENTRY && p != EXIT => {
                let other = match self.portals[&p] {
                    (a, b) if a == index => b,
                    (a, b) if b == index => a,
                    _ => panic!("portals do not match"),
                };
                let coords = self.as_coords(other);
                let new_level = match d {
                    Direction::Inward => level.map(|l| l + 1),
                    Direction::Outward => level.map(|l| l - 1),
                };
                adj.push((self.as_index(coords), new_level));
            }
            _ => (),
        };

        adj
    }

    fn walk_through(&self, recursive: bool) -> usize {
        let start = match recursive {
            true => Some(0),
            false => None,
        };
        let mut distances = HashMap::<(Option<usize>, usize), usize>::new();
        let mut queue = VecDeque::<(Option<usize>, usize, usize, usize)>::new();
        distances.insert((start, self.entry), 0);
        queue.push_front((start, 0, self.entry, self.entry));

        while let Some((level, dist, previous, current)) = queue.pop_back() {
            if current == self.exit {
                break;
            }
            let adjacent = self.adjacent(current, level);
            for &(i, ol) in adjacent.iter().filter(|&&(i, _)| i != previous) {
                if distances.get(&(ol, i)).map_or(true, |&d| dist < d) {
                    distances.insert((ol, i), dist + 1);
                    queue.push_front((ol, dist + 1, current, i));
                }
            }
        }
        distances
            .get(&(start, self.exit))
            .expect("exit not found")
            .clone()
    }
}

#[derive(Debug)]
struct ParseMapError;

impl Display for ParseMapError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "reading map failed")
    }
}

fn within_half(value: usize, range: usize) -> bool {
    value >= range / 4 && value < 3 * range / 4
}

fn find_portal(chars: &Vec<char>, width: usize, height: usize, index: usize) -> Feature {
    assert_eq!(chars[index], '.');
    let mut new_portal = None as Option<Portal>;
    let a;
    let b;

    let direction = match within_half(index % width, width) && within_half(index / width, height) {
        true => Direction::Inward,
        false => Direction::Outward,
    };

    if chars[index - 1].is_ascii_uppercase() {
        a = chars[index - 2];
        b = chars[index - 1];
        assert!(a.is_ascii_uppercase());
        new_portal = Some(Portal(a, b));
    } else if chars[index + 1].is_ascii_uppercase() {
        a = chars[index + 1];
        b = chars[index + 2];
        assert!(b.is_ascii_uppercase());
        new_portal = Some(Portal(a, b));
    } else if chars[index - width].is_ascii_uppercase() {
        a = chars[index - 2 * width];
        b = chars[index - width];
        assert!(a.is_ascii_uppercase());
        new_portal = Some(Portal(a, b));
    } else if chars[index + width].is_ascii_uppercase() {
        a = chars[index + width];
        b = chars[index + 2 * width];
        assert!(b.is_ascii_uppercase());
        new_portal = Some(Portal(a, b));
    }

    match new_portal {
        Some(portal) => Feature::Portal(portal, direction),
        None => Feature::Path,
    }
}

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();
        if !s.lines().skip(1).all(|line| line.len() == width) {
            panic!("not all lines are the same width");
        }
        let height = s.lines().count();

        let chars = s
            .lines()
            .flat_map(|line| line.chars())
            .collect::<Vec<char>>();

        let features = chars
            .iter()
            .enumerate()
            .map(|(i, &c)| match c {
                ' ' => Feature::Empty,
                '#' => Feature::Wall,
                '.' => find_portal(&chars, width, height, i),
                c if c.is_ascii_uppercase() => Feature::Empty,
                c => panic!("unrecognised character: {:?}", c),
            })
            .collect::<Vec<Feature>>();

        let portals_found = features
            .iter()
            .enumerate()
            .filter_map(|(i, f)| match f.clone() {
                Feature::Portal(p, _) => Some((i, p)),
                _ => None,
            })
            .collect::<Vec<(usize, Portal)>>();

        let mut portals = HashMap::<Portal, (usize, usize)>::new();
        let mut entry = None as Option<usize>;
        let mut exit = None as Option<usize>;

        for &(i, p) in portals_found.iter() {
            if p == ENTRY {
                match entry {
                    Some(_) => panic!("more than one entrance"),
                    None => entry = Some(i),
                }
            } else if p == EXIT {
                match exit {
                    Some(_) => panic!("more than one exit"),
                    None => exit = Some(i),
                }
            } else if !portals.contains_key(&p) {
                let others = portals_found
                    .iter()
                    .filter_map(|&(i, q)| if p == q { Some(i) } else { None })
                    .collect::<Vec<usize>>();
                if others.len() != 2 {
                    panic!("number of portals must be 2");
                }
                portals.insert(p, (others[0], others[1]));
            }
        }

        Ok(Self {
            features,
            height,
            width,
            portals,
            entry: entry.expect("no entrance found"),
            exit: exit.expect("no exit found"),
        })
    }
}

fn read_map() -> Map {
    let data = include_bytes!("input/d20.txt");
    from_utf8(data).unwrap().parse().unwrap()
}

pub fn part_a() -> String {
    let map = read_map();
    map.walk_through(false).to_string()
}

pub fn part_b() -> String {
    let map = read_map();
    map.walk_through(true).to_string()
}
