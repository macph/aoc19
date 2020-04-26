/// Advent of Code 2019, day 23
/// https://adventofcode.com/2019/day/23
use std::cell::RefCell;
use std::collections::VecDeque;
use std::iter::once;
use std::str::from_utf8;

use super::intcode::Intcode;

fn setup_intcode() -> Intcode {
    let data = include_bytes!("input/d23.txt");
    from_utf8(data).unwrap().parse::<Intcode>().unwrap()
}

#[derive(Debug)]
struct Controller {
    program: RefCell<Intcode>,
    incoming: RefCell<VecDeque<[i64; 2]>>,
}

impl Controller {
    fn start(base: &Intcode, number: i64) -> Self {
        let mut code = base.clone();
        code.run(once(number));
        assert!(!code.finished());
        Self {
            program: RefCell::new(code),
            incoming: RefCell::new(VecDeque::new()),
        }
    }

    fn is_idle(&self) -> bool {
        self.incoming.borrow().is_empty()
    }

    fn send(&self, content: [i64; 2]) {
        self.incoming.borrow_mut().push_back(content)
    }

    fn run(&self) -> Vec<(usize, [i64; 2])> {
        if !self.incoming.borrow().is_empty() {
            for packet in self.incoming.borrow_mut().drain(..) {
                self.program.borrow_mut().run(packet.iter().cloned());
            }
        } else {
            self.program.borrow_mut().run(once(-1));
        }

        let output = self.program.borrow_mut().drain().collect::<Vec<i64>>();
        assert_eq!(output.len() % 3, 0);
        output
            .chunks(3)
            .map(|p| (p[0] as usize, [p[1], p[2]]))
            .collect()
    }
}

fn run_network(terminate: bool) -> i64 {
    let size = 50usize;
    let program = setup_intcode();

    let controllers = (0..size as i64)
        .map(|i| Controller::start(&program, i))
        .collect::<Vec<Controller>>();

    let mut nat = None as Option<[i64; 2]>;
    let mut last_sent = None as Option<i64>;

    loop {
        for c in controllers.iter() {
            for (dest, content) in c.run() {
                match dest {
                    255 if terminate => return content[1],
                    255 => nat = Some(content),
                    d if d < size => controllers[d].send(content),
                    d => panic!("invalid address {}", d),
                };
            }
        }

        if !terminate && controllers.iter().all(|c| c.is_idle()) {
            let content = nat.unwrap();
            if last_sent == Some(content[1]) {
                return content[1];
            } else {
                last_sent = Some(content[1]);
            }
            controllers[0].send(content);
        }
    }
}

pub fn part_a() -> String {
    run_network(true).to_string()
}

pub fn part_b() -> String {
    run_network(false).to_string()
}
