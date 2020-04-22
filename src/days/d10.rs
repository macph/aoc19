/// Advent of Code 2019, day 10
/// https://adventofcode.com/2019/day/10
use std::f64::consts::PI;

fn gcd(a: u32, b: u32) -> u32 {
    if b > a {
        gcd(b, a)
    } else if a == b || b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

trait Vector {
    fn reduce(&self) -> Self;
    fn add(&self, rhs: Self) -> Self;
    fn angle(&self) -> f64;
}

impl Vector for (i32, i32) {
    fn reduce(&self) -> Self {
        let c = gcd(self.0.abs() as u32, self.1.abs() as u32) as i32;
        (self.0 / c, self.1 / c)
    }

    fn add(&self, rhs: Self) -> Self {
        (self.0 + rhs.0, self.1 + rhs.1)
    }

    fn angle(&self) -> f64 {
        let a = (self.0 as f64).atan2(-self.1 as f64) % (2. * PI);
        if a < 0. {
            a + 2. * PI
        } else {
            a
        }
    }
}

struct LOS {
    position: (usize, usize),
    map: Box<dyn Iterator<Item = (usize, usize)>>,
}

impl LOS {
    fn new(position: (usize, usize), width: usize, height: usize) -> LOS {
        LOS {
            position,
            map: Box::new((0..height).flat_map(move |j| (0..width).map(move |i| (i, j)))),
        }
    }

    fn find_los(&self, current: (usize, usize)) -> Option<(i32, i32)> {
        let x = current.0 as i32 - self.position.0 as i32;
        let y = current.1 as i32 - self.position.1 as i32;
        if (x != 0 || y != 0) && (x, y).reduce() == (x, y) {
            Some((x, y))
        } else {
            None
        }
    }
}

impl Iterator for LOS {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let c = self.map.next()?;
            if let Some(f) = self.find_los(c) {
                return Some(f);
            }
        }
    }
}

fn los_clockwise(position: (usize, usize), width: usize, height: usize) -> Vec<(i32, i32)> {
    let mut vectors: Vec<(i32, i32)> = LOS::new(position, height, width).collect();
    vectors.sort_by(|&u, &v| u.angle().partial_cmp(&v.angle()).unwrap());
    vectors
}

fn absolute_position(position: (usize, usize), offset: (i32, i32)) -> Option<(usize, usize)> {
    let x = position.0 as i32 + offset.0;
    let y = position.1 as i32 + offset.1;
    if x >= 0 && y >= 0 {
        Some((x as usize, y as usize))
    } else {
        None
    }
}

struct Map {
    points: Vec<bool>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(points: Vec<bool>, width: usize, height: usize) -> Map {
        if width * height != points.len() {
            panic!(
                "Vector for Map is the wrong size; should be {} * {} = {}.",
                width,
                height,
                width * height
            );
        }
        Map {
            points,
            width,
            height,
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn points(&self) -> Vec<(usize, usize)> {
        self.points
            .iter()
            .enumerate()
            .filter_map(|(i, &p)| {
                if p {
                    Some((i % self.width, i / self.width))
                } else {
                    None
                }
            })
            .collect()
    }

    fn get(&self, position: (usize, usize)) -> Option<bool> {
        if position.0 < self.width && position.1 < self.height {
            Some(self.points[position.0 + position.1 * self.width])
        } else {
            None
        }
    }

    fn find_in_sight(
        &self,
        position: (usize, usize),
        vector: (i32, i32),
    ) -> Option<(usize, usize)> {
        let base = vector.reduce();
        let mut offset = base;
        loop {
            let a = absolute_position(position, offset)?;
            if self.get(a)? {
                return Some(a);
            }
            offset = offset.add(base);
        }
    }

    fn count_in_sight(&self, position: (usize, usize)) -> usize {
        LOS::new(position, self.height, self.width)
            .filter_map(|o| self.find_in_sight(position, o))
            .count()
    }

    fn remove(&mut self, position: (usize, usize)) {
        self.points[position.0 + position.1 * self.width] = false;
    }
}

fn find_best_los(map: &Map) -> ((usize, usize), usize) {
    map.points()
        .iter()
        .map(|&p| (p, map.count_in_sight(p)))
        .max_by_key(|(_, c)| *c)
        .unwrap()
}

fn parse_string(string: String) -> Map {
    let lines: Vec<&str> = string.split('\n').collect();
    let vec: Vec<bool> = lines
        .iter()
        .flat_map(|l| {
            l.chars().map(|c| match c {
                '#' => true,
                '.' => false,
                x => panic!("{} not valid; must be '#' or '.'.", x),
            })
        })
        .collect();
    Map::new(vec, lines[0].len(), lines.len())
}

fn parse_input() -> Map {
    let data = include_bytes!("input/d10.txt");
    parse_string(String::from_utf8_lossy(data).to_string())
}

pub fn part_a() -> String {
    find_best_los(&parse_input()).1.to_string()
}

pub fn part_b() -> String {
    let mut map = parse_input();
    let position: (usize, usize) = (31, 20);
    let vectors = los_clockwise(position, map.width(), map.height());

    let mut current: (usize, usize) = (0, 0);
    let mut count = 0;
    for v in vectors.iter().cloned().cycle() {
        if let Some(p) = map.find_in_sight(position, v) {
            map.remove(p);
            current = p;
            count += 1;
        };
        if count >= 200 {
            break;
        }
    }
    (current.0 * 100 + current.1).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(54, 24), 6);
    }

    #[test]
    fn test_los() {
        let result: Vec<(i32, i32)> = LOS::new((0, 4), 5, 5).collect();
        assert_eq!(
            result,
            vec![
                (1, -4),
                (3, -4),
                (1, -3),
                (2, -3),
                (4, -3),
                (1, -2),
                (3, -2),
                (0, -1),
                (1, -1),
                (2, -1),
                (3, -1),
                (4, -1),
                (1, 0)
            ]
        );
    }

    #[test]
    fn test_los_clockwise() {
        let expected = vec![
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];
        let result: Vec<(i32, i32)> = los_clockwise((1, 1), 3, 3);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_los_point_1() {
        let input = "\
#.#.#
.#.#.
#.#.#
.#.#.
#.#.#";
        let map = parse_string(input.to_string());
        let pos: (usize, usize) = (0, 4);
        assert_eq!(map.count_in_sight(pos), 7);
    }

    #[test]
    fn test_los_1() {
        let input = "\
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        let result = find_best_los(&parse_string(input.to_string()));
        assert_eq!(result.0, (5, 8));
        assert_eq!(result.1, 33);
    }

    #[test]
    fn test_los_2() {
        let input = "\
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
        let result = find_best_los(&parse_string(input.to_string()));
        assert_eq!(result.0, (1, 2));
        assert_eq!(result.1, 35);
    }

    #[test]
    fn test_los_3() {
        let input = "\
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
        let result = find_best_los(&parse_string(input.to_string()));
        assert_eq!(result.0, (6, 3));
        assert_eq!(result.1, 41);
    }

    #[test]
    fn test_los_4() {
        let input = "\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        let result = find_best_los(&parse_string(input.to_string()));
        assert_eq!(result.0, (11, 13));
        assert_eq!(result.1, 210);
    }
}
