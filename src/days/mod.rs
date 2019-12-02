mod d01;

use d01::{d01a, d01b};

pub type Solver = dyn Fn() -> String + Send + Sync + 'static;

lazy_static! {
    pub static ref SOLVERS: Vec<Vec<&'static Solver>> = vec![vec![&d01a, &d01b],];
}
