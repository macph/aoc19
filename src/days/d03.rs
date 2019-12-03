use std::collections::HashMap;

type Coord = (i32, i32);

#[derive(Debug)]
struct Move {
    x: i32,
    y: i32,
    d: i32,
}

impl Move {
    fn from_str(cmd: &str) -> Move {
        let mut dx: i32 = 0;
        let mut dy: i32 = 0;
        match cmd.chars().nth(0).unwrap() {
            'D' => dy = -1,
            'L' => dx = -1,
            'R' => dx = 1,
            'U' => dy = 1,
            x => panic!("Move command '{}' is invalid.", x),
        }
        let dist = cmd[1..].parse::<i32>().unwrap();

        Move {
            x: dist * dx,
            y: dist * dy,
            d: dist,
        }
    }

    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn distance(&self) -> i32 {
        self.d
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    coord: Coord,
    intersections: u8,
    length: i32,
}

impl Point {
    fn new(coord: Coord) -> Point {
        Point {
            coord,
            intersections: 0,
            length: 0,
        }
    }

    fn add(&mut self, length: i32) {
        self.intersections += 1;
        self.length += length;
    }

    fn intersections(&self) -> u8 {
        self.intersections
    }

    fn length(&self) -> i32 {
        self.length
    }

    fn distance(&self) -> i32 {
        self.coord.0.abs() + self.coord.1.abs()
    }
}

#[derive(Debug)]
struct Grid {
    grid: HashMap<Coord, Point>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            grid: HashMap::new(),
        }
    }

    fn increment_with(&mut self, c: Coord, length: i32) {
        self.grid.entry(c).or_insert(Point::new(c)).add(length);
    }

    fn add_wire(&mut self, wire: &Vec<Move>) {
        let mut c: Coord = (0, 0);
        let mut t: i32 = 0;
        let mut dx: i32;
        let mut dy: i32;
        for m in wire {
            dx = m.x().signum();
            dy = m.y().signum();
            for i in 1..=m.distance() {
                self.increment_with((c.0 + i * dx, c.1 + i * dy), t + i);
            }
            c = (c.0 + m.x(), c.1 + m.y());
            t += m.distance();
        }
    }

    fn intersections(self) -> Vec<Point> {
        self.grid
            .iter()
            .filter_map(|(_, &p)| if p.intersections() > 1 { Some(p) } else { None })
            .collect()
    }
}

fn parse_input() -> Vec<Vec<Move>> {
    let data = include_bytes!("d03.txt");
    String::from_utf8_lossy(data)
        .split('\n')
        .map(|s| s.split(',').map(|c| Move::from_str(c)).collect())
        .collect()
}

pub fn d03a() -> String {
    let mut g = Grid::new();
    for w in parse_input().iter() {
        g.add_wire(w);
    }
    g.intersections()
        .iter()
        .map(|&p| p.distance())
        .min()
        .unwrap()
        .to_string()
}

pub fn d03b() -> String {
    let mut g = Grid::new();
    for w in parse_input().iter() {
        g.add_wire(w);
    }
    g.intersections()
        .iter()
        .map(|&p| p.length())
        .min()
        .unwrap()
        .to_string()
}
