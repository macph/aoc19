/// Advent of Code 2019, day 22
/// https://adventofcode.com/2019/day/22
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::{from_utf8, FromStr};

// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut r0, mut r1) = (a, b);
    let (mut s0, mut s1) = (1, 0);
    let (mut t0, mut t1) = (0, 1);
    let mut q;

    let update = |q, v0: &mut i64, v1: &mut i64| {
        let p = *v1;
        *v1 = *v0 - q * *v1;
        *v0 = p;
    };
    while r1 != 0 {
        q = r0.div_euclid(r1);
        update(q, &mut r0, &mut r1);
        update(q, &mut s0, &mut s1);
        update(q, &mut t0, &mut t1);
    }

    (r0, s0, t0)
}

#[derive(Copy, Clone)]
struct Mod {
    value: i64,
    modulus: i64,
}

impl Mod {
    pub fn new(value: i64, modulus: i64) -> Self {
        if modulus == 0 {
            panic!("modulus must not be zero");
        }
        Self {
            value: value % modulus,
            modulus,
        }
    }

    fn check(&self, other: Self) {
        if self.modulus != other.modulus {
            panic!("moduli expected to be equal, got {} != {}",)
        }
    }

    // https://en.wikipedia.org/wiki/Modular_exponentiation
    pub fn pow(self, exponent: u64) -> Self {
        if self.modulus == 1 {
            return Self { value: 0, ..self };
        }
        let m = self.modulus as i128;
        let mut base = self.value as i128 % m;
        let mut exp = exponent;
        let mut result = 1;
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % m;
            }
            exp >>= 1;
            base = base * base % m;
        }
        Self {
            value: result as i64,
            ..self
        }
    }

    pub fn positive(mut self) -> Self {
        if self.value < 0 {
            self.value += self.modulus;
        }
        self
    }

    pub fn value(&self) -> i64 {
        self.value
    }

    pub fn modulus(&self) -> i64 {
        self.modulus
    }
}

impl Debug for Mod {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} mod {}", self.value, self.modulus)
    }
}

impl PartialEq for Mod {
    fn eq(&self, other: &Self) -> bool {
        self.modulus == other.modulus && self.positive().value() == other.positive().value()
    }
}

impl Eq for Mod {}

impl Add for Mod {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.check(rhs);
        self + rhs.value
    }
}

impl Add<i64> for Mod {
    type Output = Self;

    fn add(self, rhs: i64) -> Self::Output {
        let new = (self.value as i128 + rhs as i128) % self.modulus as i128;
        Self {
            value: new as i64,
            ..self
        }
    }
}

impl Sub for Mod {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.check(rhs);
        self - rhs.value
    }
}

impl Sub<i64> for Mod {
    type Output = Self;

    fn sub(self, rhs: i64) -> Self::Output {
        let new = (self.value as i128 - rhs as i128) % self.modulus as i128;
        Self {
            value: new as i64,
            ..self
        }
    }
}

impl Neg for Mod {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            value: -self.value,
            ..self
        }
    }
}

impl Mul for Mod {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.check(rhs);
        self * rhs.value
    }
}

impl Mul<i64> for Mod {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        let new = (self.value as i128 * rhs as i128) % self.modulus as i128;
        Self {
            value: new as i64,
            ..self
        }
    }
}

impl Div for Mod {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.check(rhs);
        self / rhs.value
    }
}

impl Div<i64> for Mod {
    type Output = Self;

    fn div(self, rhs: i64) -> Self::Output {
        // https://en.wikipedia.org/wiki/Modular_multiplicative_inverse
        let (g, inv, _) = extended_gcd(rhs, self.modulus);
        if g != 1 {
            panic!("gcd between denominator and modulus must be 1, ie both numbers are coprime");
        }
        self * inv
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct ModLinearExpression {
    a: Mod,
    b: Mod,
}

impl ModLinearExpression {
    fn new(m: i64, a: i64, b: i64) -> Self {
        Self {
            a: Mod::new(a, m),
            b: Mod::new(b, m),
        }
    }

    fn base(m: i64) -> Self {
        Self::new(m, 1, 0)
    }

    fn modulus(&self) -> i64 {
        if self.a.modulus() != self.b.modulus() {
            panic!("expected moduli to be the same");
        }
        self.a.modulus()
    }

    fn apply(&self, x: i64, invert: bool) -> i64 {
        let r = if invert {
            (-self.b + x) / self.a
        } else {
            self.a * x + self.b
        };
        r.positive().value()
    }

    fn compose(&self, inner: Self) -> Self {
        if self.modulus() != inner.modulus() {
            panic!(
                "expected moduli to be equal, got {} != {}",
                self.modulus(),
                inner.modulus()
            );
        }
        Self {
            a: self.a * inner.a,
            b: self.a * inner.b + self.b,
        }
    }

    fn compose_self(&self, depth: u64) -> Self {
        let modulus = self.modulus();
        if depth == 0 {
            return Self::base(modulus);
        } else if depth == 1 {
            return self.clone();
        }

        let pow_a = self.a.pow(depth);
        // Geometric series sum
        let sum = if self.a.value() != 1 {
            (-pow_a + 1) / (-self.a + 1)
        } else {
            Mod::new(depth as i64, modulus)
        };
        Self {
            a: pow_a,
            b: self.b * sum,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Stack,
    Cut(i64),
    Deal(i64),
}

impl Instruction {
    fn as_linear(&self, m: i64) -> ModLinearExpression {
        match *self {
            Instruction::Stack => ModLinearExpression::new(m, -1, -1),
            Instruction::Cut(n) => ModLinearExpression::new(m, 1, -n),
            Instruction::Deal(n) => ModLinearExpression::new(m, n, 0),
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const STACK: &'static str = "deal into new stack";
        const CUT: &'static str = r"cut ";
        const INCREMENT: &'static str = "deal with increment ";

        let i = if s == STACK {
            Instruction::Stack
        } else if s.starts_with(CUT) {
            let n = s.trim_start_matches(CUT).parse::<i64>().unwrap();
            Instruction::Cut(n)
        } else if s.starts_with(INCREMENT) {
            let n = s.trim_start_matches(INCREMENT).parse::<i64>().unwrap();
            Instruction::Deal(n)
        } else {
            Err(())?
        };
        Ok(i)
    }
}

fn read_instructions() -> Vec<Instruction> {
    let data = include_bytes!("input/d22.txt");
    from_utf8(data)
        .unwrap()
        .lines()
        .map(|line| line.parse::<Instruction>())
        .collect::<Result<Vec<Instruction>, ()>>()
        .unwrap()
}

fn compose_instructions(m: i64, ins: &[Instruction]) -> ModLinearExpression {
    ins.iter().fold(ModLinearExpression::base(m), |a, b| {
        b.as_linear(a.modulus()).compose(a)
    })
}

pub fn part_a() -> String {
    let size = 10007;
    let card = 2019;

    let expression = compose_instructions(size, &read_instructions());
    expression.apply(card, false).to_string()
}

pub fn part_b() -> String {
    let size = 119315717514047;
    let repeat = 101741582076661;
    let card = 2020;

    let base = compose_instructions(size, &read_instructions());
    let expression = base.compose_self(repeat);
    expression.apply(card, true).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compose() {
        let size = 11;
        let collect = |deck: &[i64]| {
            let mut new_deck = vec![0i64; size as usize];
            for (i, &c) in deck.iter().enumerate() {
                assert!(c >= 0 && c < 11);
                new_deck[c as usize] = i as i64;
            }
            new_deck
        };

        let instructions = &[
            Instruction::Stack,
            Instruction::Cut(-2),
            Instruction::Deal(7),
            Instruction::Cut(8),
        ];
        let composed = instructions
            .iter()
            .fold(ModLinearExpression::base(size), |a, b| {
                b.as_linear(a.modulus()).compose(a)
            });
        assert_eq!(composed, ModLinearExpression::new(size, -7, -1));

        let new = (0..size)
            .map(|c| composed.apply(c, false))
            .collect::<Vec<i64>>();
        assert_eq!(collect(&new), [3, 6, 9, 1, 4, 7, 10, 2, 5, 8, 0]);
    }

    #[test]
    fn test_self_compose() {
        let size = 11;
        let expression = ModLinearExpression::new(size, -7, -1);
        assert_eq!(
            expression.compose_self(1),
            ModLinearExpression::new(size, -7, -1)
        );
        assert_eq!(
            expression.compose_self(2),
            ModLinearExpression::new(size, 5, 6)
        );
    }
}
