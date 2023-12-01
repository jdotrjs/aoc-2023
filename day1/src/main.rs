use std::fs;
use regex::Regex;

// The newly-improved calibration document consists of lines of text; each line
// originally contained a specific calibration value that the Elves now need to
// recover. On each line, the calibration value can be found by combining the
// first digit and the last digit (in that order) to form a single two-digit
// number.
//
// In this example, the calibration values of these four lines are 12, 38, 15,
// and 77. Adding these together produces 142.

fn is_numeric(input: char) -> bool {
    return '0' <= input && '9' >= input;
}

fn as_numeric(input: &str) -> u32 {
    if (is_numeric(char::from_u32(u32::from(input.as_bytes()[0])).unwrap())) {
        return input.parse().expect("a number");
    }
    match input {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("expected a number"),
    }
}

fn opt_to_string<T: std::fmt::Display>(o: Option<T>) -> String {
    match o {
        Some(v) => std::fmt::format(format_args!("Some({})", v.to_string())),
        None => String::from("None"),
    }
}

fn part1(l: &str) -> Option<(u32, u32)> {
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;

    for c in l.chars() {
        if is_numeric(c) {
            if None == first {
                first = c.to_digit(10);
            }
            last = c.to_digit(10);
        }
    }

    if let Some(_) = first {
        return Some((first.unwrap(), last.unwrap()));
    }
    return None;
}

fn part2(first_re: &Regex, last_re: &Regex, l: &str) -> Option<(u32, u32)> {
    // println!("{}", l);

    let find_res = first_re.find(l);
    if let Some(first) = find_res {
        // println!("First match {} -> {}", first.start(), first.as_str());
        let first_u32: u32 = as_numeric(first.as_str());

        // this doesn't work for `twone` which should break but is fine by accident
        let last_res = last_re.captures_at(l, first.end());
        if let Some(last) = last_res {
            if let Some(m) = last.name("n") {
                // println!("Some: ({}, {})", first_u32, as_numeric(m.as_str()));
                return Some((first_u32, as_numeric(m.as_str())));
            }
        } else {
            // println!("Some({}, {})", first_u32, first_u32);
            return Some((first_u32, first_u32));
        }
    }

    // println!("None");
    return None
}

fn main() {
    let sample_input: Vec<&str> = vec![
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "no numbers here",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
        "ntfpgz1x",
    ];

    let contents = fs::read_to_string("input.txt").expect("Wanted input");
    let real_input = contents.split("\n");

    let mut coords: Vec<u32> = vec![];

    let first_re = Regex::new("([0-9]|one|two|three|four|five|six|seven|eight|nine)").expect("working regex");
    let last_re = Regex::new(".*(?<n>[0-9]|one|two|three|four|five|six|seven|eight|nine)").expect("working regex");

    for line in real_input {
        // println!("{}", line);
        match part2(&first_re, &last_re, line) {
            None => (),
            Some((f, l)) => {
                let coord = std::fmt::format(format_args!("{}{}", f, l));
                let parse_result: u32 = coord.parse().expect("A number");
                // println!("{} / {}", f, l);
                coords.push(parse_result);
            },
        }
    }
    
    let mut sum = 0;
    for c in coords.iter() {
        sum += c;
    }

    println!("Coord: {}", sum);
}