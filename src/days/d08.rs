/// Advent of Code 2019, day 8
/// https://adventofcode.com/2019/day/8
use itertools::Itertools;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn byte_to_int(i: u8) -> u8 {
    if i >= 48 && i < 59 {
        i - 48
    } else {
        panic!("Byte {:#x} is not an ASCII integer.", i);
    }
}

fn read_input() -> Vec<u8> {
    let data = include_bytes!("input/d08.txt");
    data.iter().map(|&i| byte_to_int(i)).collect()
}

pub fn d08a() -> String {
    let layer = read_input()
        .chunks(WIDTH * HEIGHT)
        .min_by_key(|layer| layer.iter().filter(|&&i| i == 0).count())
        .unwrap()
        .to_vec();
    let ones = layer.iter().filter(|&&i| i == 1).count();
    let twos = layer.iter().filter(|&&i| i == 2).count();
    (ones * twos).to_string()
}

pub fn d08b() -> String {
    let layers: Vec<Vec<u8>> = read_input()
        .chunks(WIDTH * HEIGHT)
        .map(|layer| layer.to_vec())
        .collect();
    let image: Vec<u8> = (0..WIDTH * HEIGHT)
        .map(|p| {
            (0..layers.len())
                .map(|i| layers[i][p])
                .skip_while(|&c| c == 2)
                .nth(0)
                .unwrap()
        })
        .collect();
    image
        .chunks(WIDTH)
        .map(|l| l.iter().map(|&p| if p == 1 { "#" } else { " " }).join(" "))
        .join("\n")
}
