fn parse_input() -> Vec<i32> {
    let data = include_bytes!("d01.txt");
    String::from_utf8_lossy(data)
        .split('\n')
        .filter_map(|s| match s.parse::<i32>() {
            Ok(m) => Some(m),
            Err(_) => None,
        })
        .collect()
}

fn fuel_required(mass: i32) -> i32 {
    mass / 3 - 2
}

pub fn d01a() -> String {
    parse_input()
        .iter()
        .map(|&mass| fuel_required(mass))
        .sum::<i32>()
        .to_string()
}

pub fn d01b() -> String {
    parse_input()
        .iter()
        .map(|&mass| {
            let mut m = mass;
            let mut total = 0;
            loop {
                m = fuel_required(m);
                if m <= 0 {
                    break;
                }
                total += m;
            }
            total
        })
        .sum::<i32>()
        .to_string()
}
