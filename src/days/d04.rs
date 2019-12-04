/// Advent of Code 2019, day 4
/// https://adventofcode.com/2019/day/4

const MIN: u32 = 357253;
const MAX: u32 = 892942;

struct IterDigits {
    remaining: u32,
    finished: bool,
}

impl Iterator for IterDigits {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else {
            let x = self.remaining % 10;
            self.remaining /= 10;
            if self.remaining == 0 {
                self.finished = true;
            }
            Some(x as Self::Item)
        }
    }
}

fn iter_digits(integer: u32) -> IterDigits {
    IterDigits {
        remaining: integer,
        finished: false,
    }
}

pub fn d04a() -> String {
    (MIN..=MAX)
        .filter(|&pw| {
            let mut p: Option<u8> = None;
            let mut adj = false;
            for d in iter_digits(pw) {
                if p.is_some() {
                    if p.unwrap() < d {
                        return false;
                    }
                    if p.unwrap() == d {
                        adj = true;
                    }
                }
                p = Some(d);
            }
            adj
        })
        .count()
        .to_string()
}

pub fn d04b() -> String {
    (MIN..=MAX)
        .filter(|&pw| {
            let mut p: Option<u8> = None;
            let mut pairs: Vec<u8> = Vec::new();
            for d in iter_digits(pw) {
                if p.is_some() {
                    if p.unwrap() < d {
                        return false;
                    }
                    if p.unwrap() == d {
                        pairs.push(d);
                    }
                }
                p = Some(d);
            }
            pairs
                .iter()
                .map(|&p| pairs.iter().filter(|&&q| p == q).count())
                .any(|c| c == 1)
        })
        .count()
        .to_string()
}
