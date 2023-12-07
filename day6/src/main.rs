use std::env;
use regex::Regex;
use once_cell::sync::Lazy;
// use std::collections::HashMap;

const NUMBERS_RE: Lazy<Regex> = Lazy::new(||
    Regex::new(r".*:(.*)").expect("regex")
);

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn winning_times(&self) -> Vec<u64> {
        // Your toy boat has a starting speed of zero millimeters per
        // millisecond. For each whole millisecond you spend at the
        // beginning of the race holding down the button, the boat's
        // speed increases by one millimeter per millisecond.
        let mut v = vec![];
        for charge_time in 0..self.time {
            if self.distance < (charge_time * (self.time - charge_time)) {
                v.push(charge_time);
            }
        }
        return v;
    }
}

struct Day6 {}

impl Day6 {
    fn parse_input(input: &String) -> Vec<Race> {
        let mut line_iter = input.lines();

        let times = num_str_to_vec(NUMBERS_RE.captures(line_iter.next().unwrap()).unwrap().get(1).unwrap().as_str());
        let distances = num_str_to_vec(NUMBERS_RE.captures(line_iter.next().unwrap()).unwrap().get(1).unwrap().as_str());

        let mut races = vec![];
        for idx in 0..(times.len()) {
            races.push(Race{
                time: *times.get(idx).unwrap(),
                distance: *distances.get(idx).unwrap(),
            });
        }
        return races;
    }

    fn parse_input2(input: &String) -> Race {
        let v = Day6::parse_input(input);
        let mut time = String::new();
        let mut dist = String::new();
        for r in v {
            time.push_str(r.time.to_string().as_str());
            dist.push_str(r.distance.to_string().as_str());
        }

        Race { time: time.parse().expect("num"), distance: dist.parse().expect("num") }
    }
}

fn num_str_to_vec(input: &str) -> Vec<u64> {
    let mut r = Vec::new();
    for e in input.split(" ") {
        if e.len() == 0 {
            continue;
        }
        r.push(e.trim().parse().expect("a number"));
    }
    return r;
}


fn part1(input: &String) {
    let races = Day6::parse_input(input);

    let mut acc = 1;

    for r in races {
        // println!("{:?}", r);
        let wt = r.winning_times();
        // println!("{:?}", wt);
        acc = acc * wt.len();
    }

    println!("acc: {acc}");
}

fn part2(input: &String) {
    let r = Day6::parse_input2(input);
    let wt = r.winning_times();
    let acc = wt.len();
    println!("acc: {acc}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_path = String::from("test.txt");
    let fp = args.get(1).unwrap_or(&default_path);

    let input = std::fs::read_to_string(fp).expect("input");
    part1(&input);
    part2(&input);
}
