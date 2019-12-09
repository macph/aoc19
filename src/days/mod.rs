/// Collect all problems into a list to be iterated over
mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;

mod intcode;

use d01::{d01a, d01b};
use d02::{d02a, d02b};
use d03::{d03a, d03b};
use d04::{d04a, d04b};
use d05::{d05a, d05b};
use d06::{d06a, d06b};
use d07::{d07a, d07b};
use d08::{d08a, d08b};
use d09::{d09a, d09b};

pub type Solver = dyn Fn() -> String + Send + Sync + 'static;

lazy_static! {
    pub static ref SOLVERS: Vec<Vec<&'static Solver>> = vec![
        vec![&d01a, &d01b],
        vec![&d02a, &d02b],
        vec![&d03a, &d03b],
        vec![&d04a, &d04b],
        vec![&d05a, &d05b],
        vec![&d06a, &d06b],
        vec![&d07a, &d07b],
        vec![&d08a, &d08b],
        vec![&d09a, &d09b],
    ];
}
