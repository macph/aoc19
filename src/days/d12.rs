/// Advent of Code 2019, day 12
/// https://adventofcode.com/2019/day/12
use std::str::FromStr;

use itertools::Itertools;
use regex::{Captures, Regex};

lazy_static! {
    static ref RE: Regex = Regex::new("<x=(-?\\d+),\\s*y=(-?\\d+),\\s*z=(-?\\d+)>").unwrap();
}

fn gcd(a: u64, b: u64) -> u64 {
    if b > a {
        gcd(b, a)
    } else if a == b || b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn fold_lcm(v: Vec<u64>) -> u64 {
    v.iter().fold(1, |a, &e| lcm(a, e))
}

type V = i32;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Moon {
    position: (V, V, V),
    velocity: (V, V, V),
}

impl Moon {
    fn new(x: V, y: V, z: V) -> Self {
        Self {
            position: (x, y, z),
            velocity: (0, 0, 0),
        }
    }

    fn potential_energy(&self) -> V {
        self.position.0.abs() + self.position.1.abs() + self.position.2.abs()
    }

    fn kinetic_energy(&self) -> V {
        self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs()
    }

    fn total_energy(&self) -> V {
        self.potential_energy() + self.kinetic_energy()
    }

    fn gravity_effect(&self, other: &Self) -> (V, V, V) {
        (
            (other.position.0 - self.position.0).signum(),
            (other.position.1 - self.position.1).signum(),
            (other.position.2 - self.position.2).signum(),
        )
    }

    fn apply_gravity(&mut self, gravity: (V, V, V)) {
        self.velocity.0 += gravity.0;
        self.velocity.1 += gravity.1;
        self.velocity.2 += gravity.2;
    }

    fn apply_velocity(&mut self, step: V) {
        self.position.0 += step * self.velocity.0;
        self.position.1 += step * self.velocity.1;
        self.position.2 += step * self.velocity.2;
    }
}

fn parse_group(c: &Captures, i: usize) -> Result<i32, String> {
    c.get(i)
        .unwrap()
        .as_str()
        .parse::<V>()
        .map_err(|err| err.to_string())
}

impl FromStr for Moon {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match RE.captures(s) {
            Some(c) => Ok(Self::new(
                parse_group(&c, 1)?,
                parse_group(&c, 2)?,
                parse_group(&c, 3)?,
            )),
            None => Err("Vector must be in form '<x=0, y=0, z=0>'.".to_string()),
        }
    }
}

fn parse_input() -> Vec<Moon> {
    let data = include_bytes!("input/d12.txt");
    String::from_utf8_lossy(data)
        .split('\n')
        .map(|s| s.parse::<Moon>().unwrap())
        .collect()
}

fn simulate_system_step(moons: &mut Vec<Moon>) {
    for pair in (0..moons.len()).combinations(2) {
        let (i, j) = (pair[0], pair[1]);
        let g0 = moons[i].gravity_effect(&moons[j]);
        let g1 = moons[j].gravity_effect(&moons[i]);
        moons[i].apply_gravity(g0);
        moons[j].apply_gravity(g1);
    }
    for m in moons.iter_mut() {
        m.apply_velocity(1);
    }
}

pub fn part_a() -> String {
    let mut moons = parse_input();
    for _ in 0..1000 {
        simulate_system_step(&mut moons);
    }

    moons
        .iter()
        .map(|b| b.total_energy())
        .sum::<V>()
        .to_string()
}

fn x_coords(moons: &Vec<Moon>) -> Vec<V> {
    moons.iter().map(|m| m.position.0).collect()
}

fn y_coords(moons: &Vec<Moon>) -> Vec<V> {
    moons.iter().map(|m| m.position.1).collect()
}

fn z_coords(moons: &Vec<Moon>) -> Vec<V> {
    moons.iter().map(|m| m.position.2).collect()
}

fn x_velocity_zero(moons: &Vec<Moon>) -> bool {
    moons.iter().all(|m| m.velocity.0 == 0)
}

fn y_velocity_zero(moons: &Vec<Moon>) -> bool {
    moons.iter().all(|m| m.velocity.1 == 0)
}

fn z_velocity_zero(moons: &Vec<Moon>) -> bool {
    moons.iter().all(|m| m.velocity.2 == 0)
}

pub fn part_b() -> String {
    let mut moons = parse_input();
    let initial = (x_coords(&moons), y_coords(&moons), z_coords(&moons));

    let mut found: (u64, u64, u64) = (0, 0, 0);
    let mut step: u64 = 0;
    loop {
        simulate_system_step(&mut moons);
        step += 1;
        if found.0 == 0 && x_velocity_zero(&moons) && x_coords(&moons) == initial.0 {
            found.0 = step;
        }
        if found.1 == 0 && y_velocity_zero(&moons) && y_coords(&moons) == initial.1 {
            found.1 = step;
        }
        if found.2 == 0 && z_velocity_zero(&moons) && z_coords(&moons) == initial.2 {
            found.2 = step;
        }
        if found.0 > 0 && found.0 > 1 && found.2 > 0 {
            break;
        }
    }
    fold_lcm(vec![found.0, found.1, found.2]).to_string()
}
