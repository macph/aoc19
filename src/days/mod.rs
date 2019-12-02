mod d01;
mod d02;

use d01::{d01a, d01b};
use d02::{d02a, d02b};

pub type Solver = dyn Fn() -> String + Send + Sync + 'static;

lazy_static! {
    pub static ref SOLVERS: Vec<Vec<&'static Solver>> =
        vec![vec![&d01a, &d01b], vec![&d02a, &d02b],];
}
