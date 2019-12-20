/// Advent of Code 2019, day 18
/// https://adventofcode.com/2019/day/18
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt;
use std::iter::{once, FromIterator};
use std::str::{from_utf8, FromStr};
use std::usize;

fn char_to_u32(c: char) -> Option<u32> {
    if c.is_ascii_uppercase() {
        Some(c as u32 - 'A' as u32)
    } else if c.is_ascii_lowercase() {
        Some(c as u32 - 'a' as u32)
    } else {
        None
    }
}

fn char_from_u32(i: u32) -> Option<char> {
    if i < 26 {
        Some((i as u8 + ('A' as u8)) as char)
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Keys(u32);

impl Keys {
    fn new() -> Self {
        Self(0)
    }

    fn bits(&self) -> usize {
        let mut field = self.0;
        let mut size = 0;
        while field > 0 {
            field /= 2;
            size += 1;
        }
        size
    }

    fn with(&self, c: char) -> Self {
        Self(self.0 | 1 << char_to_u32(c).unwrap())
    }

    fn has(&self, c: char) -> bool {
        self.0 & 1 << char_to_u32(c).unwrap() > 0
    }

    fn complete(&self) -> bool {
        let mut f = self.0;
        if f == 0 {
            return false;
        }
        while f > 0 {
            if (f & 1) == 0 {
                return false;
            }
            f >>= 1;
        }
        true
    }

    fn is_subset(&self, doors: Keys) -> bool {
        self.0 | doors.0 == self.0
    }
}

impl fmt::Display for Keys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.bits() as u32 {
            let c = char_from_u32(i).unwrap();
            write!(f, "{}", if self.has(c) { c } else { '.' })?;
        }
        write!(f, "]")
    }
}

#[derive(Debug)]
enum MapParseError {
    FeatureParseError(char),
    InvalidWidth,
    NoEntrance,
}

impl fmt::Display for MapParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapParseError::FeatureParseError(c) => write!(
                f,
                "Char {:?} is not recognised. '@', '#', '.' or ASCII letters are allowed.",
                c
            ),
            MapParseError::InvalidWidth => write!(f, "Not all lines have the same width."),
            MapParseError::NoEntrance => write!(f, "No entrance marked by '@' was found."),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Feature {
    Entrance,
    Wall,
    Floor,
    Key(char),
    Door(char),
}

impl Feature {
    fn from_char(c: char) -> Result<Feature, MapParseError> {
        match c {
            '@' => Ok(Feature::Entrance),
            '#' => Ok(Feature::Wall),
            '.' => Ok(Feature::Floor),
            _ if c.is_ascii_lowercase() => Ok(Feature::Key(c)),
            _ if c.is_ascii_uppercase() => Ok(Feature::Door(c.to_lowercase().nth(0).unwrap())),
            _ => Err(MapParseError::FeatureParseError(c)),
        }
    }

    fn passable(&self) -> bool {
        match self {
            Feature::Wall => false,
            _ => true,
        }
    }
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Feature::Entrance => '@',
            Feature::Wall => '#',
            Feature::Floor => '.',
            Feature::Key(c) => *c,
            Feature::Door(c) => c.to_uppercase().nth(0).unwrap(),
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
struct Map {
    features: Vec<Feature>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(features: Vec<Feature>, width: usize, height: usize) -> Self {
        Self {
            features,
            width,
            height,
        }
    }

    fn entrances(&self) -> Vec<usize> {
        self.features
            .iter()
            .enumerate()
            .filter_map(|(i, f)| match f {
                Feature::Entrance => Some(i),
                _ => None,
            })
            .collect()
    }

    fn adjacent(&self, p: usize) -> Vec<usize> {
        let mut adj = Vec::new();
        if p / self.width > 0 {
            adj.push(p - self.width);
        }
        if p / self.width < self.height - 1 {
            adj.push(p + self.width);
        }
        if p % self.width > 0 {
            adj.push(p - 1);
        }
        if p % self.width < self.width - 1 {
            adj.push(p + 1);
        }
        adj
    }

    fn find_keys(&self, p: usize) -> MapBFS {
        MapBFS::new(self, p)
    }

    fn get_key(&self, p: usize) -> Option<char> {
        match self.features[p] {
            Feature::Key(c) => Some(c),
            _ => None,
        }
    }

    fn replace_entrance(&mut self) {
        let entrance = {
            let e = self.entrances();
            if e.len() != 1 {
                panic!("Replacing the entrance only works when there is a single entrance!");
            }
            e[0]
        };
        let (above, below) = (entrance - self.width, entrance + self.width);

        self.features[entrance] = Feature::Wall;
        for &s in [above, entrance - 1, entrance + 1, below].iter() {
            self.features[s] = match self.features[s] {
                Feature::Floor => Feature::Wall,
                _ => panic!("{} not a floor.", s),
            }
        }
        for &d in [above - 1, above + 1, below - 1, below + 1].iter() {
            self.features[d] = match self.features[d] {
                Feature::Floor => Feature::Entrance,
                _ => panic!("{} not a floor.", d),
            }
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ls in self.features.chunks(self.width) {
            for (i, feature) in ls.iter().enumerate() {
                write!(f, "{}", feature)?;
                if i < self.width - 1 {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Map {
    type Err = MapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split('\n').collect();
        let width = lines[0].len();
        let height = lines.len();
        if !lines.iter().all(|&l| l.len() == width) {
            return Err(MapParseError::InvalidWidth);
        }
        let features = lines
            .iter()
            .flat_map(|&l| l.chars().map(|b| Feature::from_char(b)))
            .collect::<Result<Vec<Feature>, Self::Err>>()?;
        if !features.iter().any(|f| match f {
            Feature::Entrance => true,
            _ => false,
        }) {
            Err(MapParseError::NoEntrance)?
        }
        Ok(Self::new(features, width, height))
    }
}

struct MapBFS<'a> {
    map: &'a Map,
    visited: HashSet<usize>,
    queue: VecDeque<(usize, usize, Keys)>,
}

impl<'a> MapBFS<'a> {
    fn new(map: &'a Map, position: usize) -> Self {
        let first = (position, 0, Keys::new());
        Self {
            map,
            visited: HashSet::from_iter(once(position)),
            queue: VecDeque::from_iter(once(first)),
        }
    }
}

impl<'a> Iterator for MapBFS<'a> {
    type Item = (usize, usize, Keys);

    fn next(&mut self) -> Option<Self::Item> {
        let mut found: Option<Self::Item> = None;
        while found.is_none() {
            let (p0, count, k) = self.queue.pop_back()?;
            for p in self.map.adjacent(p0) {
                if self.visited.contains(&p) {
                    continue;
                }
                self.visited.insert(p);
                match self.map.features[p] {
                    Feature::Key(c) => {
                        found = Some((p, count + 1, k));
                        self.queue.push_front((p, count + 1, k.with(c)));
                    }
                    Feature::Door(c) => {
                        self.queue.push_front((p, count + 1, k.with(c)));
                    }
                    f if f.passable() => {
                        self.queue.push_front((p, count + 1, k));
                    }
                    _ => (),
                }
            }
        }
        found
    }
}

fn build_adj_matrix(map: &Map) -> HashMap<usize, HashMap<usize, (Keys, usize)>> {
    let mut matrix = HashMap::new();
    let iter_keys = map
        .features
        .iter()
        .enumerate()
        .filter_map(|(i, f)| match f {
            Feature::Key(_) => Some(i),
            _ => None,
        });
    for p in map.entrances().iter().cloned().chain(iter_keys) {
        for (q, c, k) in map.find_keys(p) {
            matrix.entry(p).or_insert(HashMap::new()).insert(q, (k, c));
        }
    }
    matrix
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Path {
    position: usize,
    dist: usize,
    keys: Keys,
}

impl Path {
    fn new(position: usize, dist: usize, keys: Keys) -> Self {
        Self {
            position,
            dist,
            keys,
        }
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn map_dijkstra(map: &Map) -> Option<usize> {
    let src = map.entrances();
    let matrix = build_adj_matrix(map);

    let mut heap = BinaryHeap::<Path>::new();
    let mut dist = HashMap::<(Keys, usize), usize>::new();

    for &i in matrix.keys() {
        let new_keys = Keys::new();
        let new_dist = if src.contains(&i) { 0 } else { usize::MAX };
        heap.push(Path::new(i, new_dist, Keys::new()));
        dist.insert((new_keys, i), new_dist);
    }

    while let Some(path) = heap.pop() {
        for (q, (k, d)) in matrix[&path.position].iter() {
            let c = map.get_key(*q).unwrap();
            if path.keys.has(c) || !path.keys.is_subset(*k) {
                continue;
            }
            let new = Path::new(*q, path.dist.saturating_add(*d), path.keys.with(c));
            let key = (new.keys, new.position);
            if !dist.contains_key(&key) || new.dist < dist[&key] {
                dist.insert((new.keys, new.position), new.dist);
                heap.push(new);
            }
        }
    }

    dist.iter()
        .filter_map(|((k, _), &d)| match k.complete() {
            true => Some(d),
            false => None,
        })
        .min()
}

fn parse_map() -> Map {
    let data = include_bytes!("input/d18.txt");
    from_utf8(data).unwrap().parse::<Map>().unwrap()
}

pub fn d18a() -> String {
    map_dijkstra(&parse_map()).unwrap().to_string()
}

pub fn d18b() -> String {
    let mut map = parse_map();
    map.replace_entrance();
    println!("{}", map);
    println!("{:?}", build_adj_matrix(&map));
    "".to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_shortest_path() {
        let data = "\
                    ########################\n\
                    #@..............ac.GI.b#\n\
                    ###d#e#f################\n\
                    ###A#B#C################\n\
                    ###g#h#i################\n\
                    ########################";
        let map = data.parse::<Map>().unwrap();
        assert_eq!(map_dijkstra(&map), Some(81));
    }
}
