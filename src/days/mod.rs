/// Collect all problems into a list to be iterated over
mod d01;
mod d02;
mod d03;
mod d04;

use d01::{d01a, d01b};
use d02::{d02a, d02b};
use d03::{d03a, d03b};
use d04::{d04a, d04b};

pub type Solver = dyn Fn() -> String + Send + Sync + 'static;

lazy_static! {
    pub static ref SOLVERS: Vec<Vec<&'static Solver>> = vec![
        vec![&d01a, &d01b],
        vec![&d02a, &d02b],
        vec![&d03a, &d03b],
        vec![&d04a, &d04b],
    ];
}
