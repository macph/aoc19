/// Advent of Code 2019, day 6
/// https://adventofcode.com/2019/day/6
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::iter::{empty, once, FromIterator};

#[derive(Debug)]
struct Graph<T>
where
    T: Eq + Hash,
{
    nodes: HashMap<T, HashSet<T>>,
}

impl<T> Graph<T>
where
    T: Eq + Hash,
{
    fn new() -> Graph<T> {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn add_edge(&mut self, a: T, b: T) {
        self.nodes
            .entry(a)
            .or_insert_with(|| HashSet::new())
            .insert(b);
    }

    fn parent<'a>(&'a self, node: &'a T) -> Option<&'a T> {
        self.nodes
            .iter()
            .filter(|&(_, b)| b.contains(node))
            .map(|(a, _)| a)
            .nth(0)
    }

    fn children<'a>(&'a self, node: &'a T) -> Box<dyn Iterator<Item = &'a T> + 'a> {
        match self.nodes.get(node) {
            Some(f) => Box::new(f.iter()),
            _ => Box::new(empty()),
        }
    }

    fn walk<'a>(&'a self, node: &'a T) -> GraphWalk<'a, T> {
        GraphWalk::new(self, node)
    }

    fn shared<'a>(&'a self, a: &'a T, b: &'a T) -> Option<&'a T> {
        let mut p = Some(a);
        while p.is_some() {
            if p.unwrap() == b || self.walk(p.unwrap()).any(|(_, q)| b == q) {
                break;
            }
            p = self.parent(p.unwrap());
        }
        p
    }

    fn distance(&self, a: &T, b: &T) -> Option<usize> {
        Some(
            self.walk(self.shared(a, b)?)
                .filter_map(|(d, n)| if n == a || n == b { Some(d) } else { None })
                .sum(),
        )
    }
}

impl<T> FromIterator<(T, T)> for Graph<T>
where
    T: Eq + Hash,
{
    fn from_iter<I: IntoIterator<Item = (T, T)>>(iter: I) -> Self {
        let mut graph = Self::new();
        for (a, b) in iter {
            graph.add_edge(a, b);
        }
        graph
    }
}

struct GraphWalk<'a, T>
where
    T: Eq + Hash,
{
    graph: &'a Graph<T>,
    remaining: VecDeque<(usize, &'a T)>,
    found: HashSet<&'a T>,
}

impl<'a, T> GraphWalk<'a, T>
where
    T: Eq + Hash,
{
    fn new(graph: &'a Graph<T>, node: &'a T) -> GraphWalk<'a, T> {
        GraphWalk {
            graph,
            remaining: VecDeque::from_iter(once((0, node))),
            found: HashSet::new(),
        }
    }
}

impl<'a, T> Iterator for GraphWalk<'a, T>
where
    T: Eq + Hash,
{
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let mut v: (usize, &T) = self.remaining.pop_front()?;
        while self.found.contains(v.1) {
            v = self.remaining.pop_front()?;
        }
        self.found.insert(v.1);

        if self.graph.nodes.contains_key(v.1) {
            for n in self.graph.children(v.1) {
                self.remaining.push_back((v.0 + 1, n));
            }
        }

        Some(v)
    }
}

fn parse_graph() -> Graph<String> {
    let data = include_bytes!("input/d06.txt");
    Graph::from_iter(String::from_utf8_lossy(data).split('\n').map(|l| {
        let mut i = l.split(')').map(|s| s.to_string());
        (i.next().unwrap(), i.next().unwrap())
    }))
}

fn count_edges(g: &Graph<String>, start: &String) -> usize {
    g.walk(start)
        .map(|(_, n)| g.walk(n).count() - 1)
        .sum::<usize>()
}

pub fn d06a() -> String {
    count_edges(&parse_graph(), &"COM".to_string()).to_string()
}

pub fn d06b() -> String {
    let g = parse_graph();
    let y = "YOU".to_string();
    let s = "SAN".to_string();
    let yp = g.parent(&y).unwrap();
    let sp = g.parent(&s).unwrap();
    g.distance(yp, sp).unwrap().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_graph() -> Graph<String> {
        Graph::from_iter(
            vec![
                ("COM", "B"),
                ("B", "C"),
                ("C", "D"),
                ("D", "E"),
                ("E", "F"),
                ("B", "G"),
                ("G", "H"),
                ("D", "I"),
                ("E", "J"),
                ("J", "K"),
                ("K", "L"),
            ]
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string())),
        )
    }

    #[test]
    fn test_graph_example() {
        assert_eq!(count_edges(&get_test_graph(), &"COM".to_string()), 42)
    }

    #[test]
    fn test_graph_shared() {
        let b = "B".to_string();
        let e = "E".to_string();
        let h = "H".to_string();
        assert_eq!(get_test_graph().shared(&e, &h), Some(&b));
    }

    #[test]
    fn test_graph_distance() {
        let e = "E".to_string();
        let h = "H".to_string();
        assert_eq!(get_test_graph().distance(&e, &h), Some(5))
    }
}
