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
mod d19;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;
mod d25;

mod intcode;

pub type Solver = dyn Fn() -> String + Send + Sync + 'static;

lazy_static! {
    pub static ref SOLVERS: Vec<Vec<&'static Solver>> = vec![
        vec![&d01::part_a, &d01::part_b],
        vec![&d02::part_a, &d02::part_b],
        vec![&d03::part_a, &d03::part_b],
        vec![&d04::part_a, &d04::part_b],
        vec![&d05::part_a, &d05::part_b],
        vec![&d06::part_a, &d06::part_b],
        vec![&d07::part_a, &d07::part_b],
        vec![&d08::part_a, &d08::part_b],
        vec![&d09::part_a, &d09::part_b],
        vec![&d10::part_a, &d10::part_b],
        vec![&d11::part_a, &d11::part_b],
        vec![&d12::part_a, &d12::part_b],
        vec![&d13::part_a, &d13::part_b],
        vec![&d14::part_a, &d14::part_b],
        vec![&d15::part_a, &d15::part_b],
        vec![&d16::part_a, &d16::part_b],
        vec![&d17::part_a, &d17::part_b],
        vec![&d18::part_a, &d18::part_b],
        vec![&d19::part_a, &d19::part_b],
        vec![&d20::part_a, &d20::part_b],
        vec![&d21::part_a, &d21::part_b],
        vec![&d22::part_a, &d22::part_b],
        vec![&d23::part_a, &d23::part_b],
        vec![&d24::part_a, &d24::part_b],
        vec![&d25::part_a],
    ];
}
