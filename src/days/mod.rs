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
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;

mod intcode;

use d01::*;
use d02::*;
use d03::*;
use d04::*;
use d05::*;
use d06::*;
use d07::*;
use d08::*;
use d09::*;
use d10::*;
use d11::*;
use d12::*;
use d13::*;
use d14::*;
use d15::*;
use d16::*;
use d17::*;
use d18::*;

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
        vec![&d10a, &d10b],
        vec![&d11a, &d11b],
        vec![&d12a, &d12b],
        vec![&d13a, &d13b],
        vec![&d14a, &d14b],
        vec![&d15a, &d15b],
        vec![&d16a, &d16b],
        vec![&d17a, &d17b],
        vec![&d18a, &d18b],
    ];
}
