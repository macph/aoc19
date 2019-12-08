#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;

use std::time::Instant;

use clap::{App, Arg};

mod days;
use days::SOLVERS;

fn format_duration(nanoseconds: u128, threshold: u128) -> String {
    if nanoseconds < threshold {
        format!("{} ns", nanoseconds)
    } else {
        let microseconds = nanoseconds / 1000;
        if microseconds < threshold {
            format!("{} µs", microseconds)
        } else {
            let milliseconds = microseconds / 1000;
            if milliseconds < threshold {
                format!("{} ms", milliseconds)
            } else {
                let seconds = milliseconds / 1000;
                format!("{} s", seconds)
            }
        }
    }
}

fn solve_problems(days: Vec<usize>) {
    let mut total_elapsed: u128 = 0;
    for (i, day) in SOLVERS
        .iter()
        .enumerate()
        .filter(|(i, _)| days.contains(&(i + 1)))
    {
        for (j, solver) in day.iter().enumerate() {
            let now = Instant::now();
            let result = solver();
            let duration = now.elapsed().as_nanos();
            let padding = if result.contains("\n") { "\n" } else { " " };
            println!(
                "Day {}, part {}:{}{}{}({})",
                i + 1,
                j + 1,
                padding,
                result,
                padding,
                format_duration(duration, 2000)
            );
            total_elapsed += duration;
        }
    }
    println!("\nTotal elapsed: {}", format_duration(total_elapsed, 2000));
}

fn main() {
    let d = SOLVERS.len();
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(
            Arg::with_name("DAY")
                .help("Select days to calculate solutions for.")
                .multiple(true)
                .validator(move |v| match v.parse::<usize>() {
                    Ok(v) if v > 0 && v <= d => Ok(()),
                    _ => Err(format!("Argument must be in range [1, {}]", d)),
                }),
        );

    let matches = app.get_matches();
    let days: Vec<usize> = match matches.values_of("DAY") {
        Some(d) => d.map(|x| x.parse::<usize>().unwrap()).collect(),
        None => (1..=d).collect(),
    };

    solve_problems(days);
}
