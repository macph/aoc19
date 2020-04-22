/// Advent of Code 2019, day 16
/// https://adventofcode.com/2019/day/16
use std::str::from_utf8;

use itertools::Itertools;

type S = i32;

const CYCLES: usize = 100;
const MULTIPLIER: usize = 10_000;
const OFFSET: usize = 7;

fn optimised_transform(signal: &mut [S], offset: usize) {
    let len = signal.len();
    if offset < (offset + len) / 2 {
        panic!("Optimised transform only works when index is more than half the total length.");
    }
    let mut sum: S = 0;
    for i in (0..len).rev() {
        sum += signal[i];
        signal[i] = sum.abs() % 10;
    }
}

fn transform_signal(signal: &mut [S], offset: usize) {
    let len = signal.len();
    for i in 0..len {
        signal[i] = signal[i..]
            .chunks(offset + i + 1)
            .enumerate()
            .map(|(j, c)| match j % 4 {
                0 => c.iter().sum::<S>(),
                2 => -c.iter().sum::<S>(),
                _ => 0,
            })
            .sum::<S>()
            .abs()
            % 10;
    }
}

pub fn read_offset(signal: &[S], digits: usize) -> usize {
    (0..digits)
        .map(|i| signal[i] as usize * 10usize.pow((digits - i - 1) as u32))
        .sum::<usize>()
}

pub fn repeat_with_offset(signal: &[S], repeat: usize, offset: usize) -> Vec<S> {
    if repeat == 0 {
        Vec::new()
    } else if repeat == 1 && offset == 0 {
        signal.to_vec()
    } else {
        signal
            .iter()
            .cycle()
            .skip(offset)
            .take(signal.len() * repeat - offset)
            .cloned()
            .collect()
    }
}

fn apply_transform(signal: &[S], cycles: usize, repeat: usize, offset_digits: usize) -> Vec<S> {
    let offset = read_offset(signal, offset_digits);
    let mut data = repeat_with_offset(signal, repeat, offset);
    let transform = if offset >= (offset + data.len()) / 2 {
        optimised_transform
    } else {
        transform_signal
    };
    for _ in 0..cycles {
        transform(&mut data, offset);
    }
    data
}

fn parse_input() -> Vec<S> {
    let data = include_bytes!("input/d16.txt");
    from_utf8(data)
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as S)
        .collect()
}

pub fn part_a() -> String {
    let data = apply_transform(&parse_input(), CYCLES, 1, 0);
    data[..8].iter().join("")
}

pub fn part_b() -> String {
    let data: Vec<S> = apply_transform(&parse_input(), CYCLES, MULTIPLIER, OFFSET);
    data[..8].iter().join("")
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse(s: &str) -> Vec<S> {
        s.chars().map(|c| c.to_digit(10).unwrap() as S).collect()
    }

    #[test]
    fn test_transform() {
        let mut s = parse("12345678");
        transform_signal(&mut s, 0);
        assert_eq!(s, parse("48226158"));
        transform_signal(&mut s, 0);
        assert_eq!(s, parse("34040438"));
        transform_signal(&mut s, 0);
        assert_eq!(s, parse("03415518"));
        transform_signal(&mut s, 0);
        assert_eq!(s, parse("01029498"));
    }

    #[test]
    fn test_apply_transform_1() {
        let s = parse("80871224585914546619083218645595");
        let t = apply_transform(&s, CYCLES, 1, 0);
        assert_eq!(t[..8], parse("24176176")[..]);
    }

    #[test]
    fn test_apply_transform_2() {
        let s = parse("19617804207202209144916044189917");
        let t = apply_transform(&s, CYCLES, 1, 0);
        assert_eq!(t[..8], parse("73745418")[..]);
    }

    #[test]
    fn test_apply_transform_3() {
        let s = parse("69317163492948606335995924319873");
        let t = apply_transform(&s, CYCLES, 1, 0);
        assert_eq!(t[..8], parse("52432133")[..]);
    }

    #[test]
    fn test_transform_repeated_offset_1() {
        let s = parse("03036732577212944063491565474664");
        let t = apply_transform(&s, CYCLES, MULTIPLIER, OFFSET);
        assert_eq!(t[..8], parse("84462026")[..]);
    }

    #[test]
    fn test_transform_repeated_offset_2() {
        let s = parse("02935109699940807407585447034323");
        let t = apply_transform(&s, CYCLES, MULTIPLIER, OFFSET);
        assert_eq!(t[..8], parse("78725270")[..]);
    }

    #[test]
    fn test_transform_repeated_offset_3() {
        let s = parse("03081770884921959731165446850517");
        let t = apply_transform(&s, CYCLES, MULTIPLIER, OFFSET);
        assert_eq!(t[..8], parse("53553731")[..]);
    }
}
