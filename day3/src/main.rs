use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct LineInfo {
    line_number: u32,
    number_mapping: HashMap<u32, (u32, u32)>,
    symbols: Vec<u32>,
}

impl LineInfo {
    fn from_str(number_re: &Regex, line_number: u32, input: &str) -> LineInfo {
        let mut number_mapping = HashMap::new();

        let mut number_id: u32 = 0;

        for e in number_re.find_iter(input) {
            let value: u32 = e.as_str().parse().expect("number");

            for idx in e.start()..e.end() {
                number_mapping.insert(u32::try_from(idx).unwrap(), (number_id, value));
            }
            
            number_id = number_id + 1;
        }

        let mut symbols = vec![];

        for idx in 0..input.len() {
            let b = input.as_bytes();
            if !b[idx].is_ascii_digit() && char::from(b[idx]) != '.' {
                symbols.push(u32::try_from(idx).unwrap());
            }
        }

        LineInfo{
            line_number,
            number_mapping,
            symbols,
        }
    }
}

fn check(tgt: &LineInfo, idx: u32, seen_numbers: &mut HashMap<u32, bool>) -> Option<(u32, u32)> {
    if let Some((id, num)) = tgt.number_mapping.get(&idx) {
        if seen_numbers.contains_key(id) {
            return None;
        }
        seen_numbers.insert(*id, true);
        return Some((*id, *num));
    }
    None
}

fn part2() {
    let sample_input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let sample_input2 = r".....
1*4..
..1..
.....
.....
..10.
1+...
";

    let real_input = std::fs::read_to_string("input.txt").expect("contents");

    let re = &Regex::new(r"([0-9]+)").expect("a regex");
    let mut line_no: u32 = 0;

    let mut last_lni: Option<LineInfo> = None;
    let mut cur_lni: Option<LineInfo> = None;
    let mut next_lni: Option<LineInfo> = None;

    let mut accum = 0;

    // for l in sample_input.split("\n") {
    // for l in sample_input2.split("\n") {
    for l in real_input.split("\n") {
        // println!("\n----------------------\n");
        next_lni = Some(LineInfo::from_str(re, line_no, l));

        // println!("last: {:?}\ncur:  {:?}\nnext: {:?}",
        //     last_lni.as_ref().map(|lni| lni.line_number),
        //     cur_lni.as_ref().map(|lni| lni.line_number),
        //     next_lni.as_ref().map(|lni| lni.line_number),
        // );

        if let Some(ref cur) = cur_lni {
            // println!("{:?}", cur);
            for symbol_idx_idx in 0..cur.symbols.len() {
                let symbol_idx = cur.symbols[symbol_idx_idx];
                // println!("processing: {}", cur.symbols[symbol_idx_idx]);

                let mut adj_num = vec![];

                if let Some(ref last) = last_lni {
                    let mut seen_numbers: HashMap<u32, bool> = HashMap::new();

                    if let Some((id, num)) = check(last, symbol_idx - 1, &mut seen_numbers) {
                        adj_num.push(num);
                        // println!("Adding ({}, {}) / {}", last.line_number, id, num);
                    }
                    if let Some((id, num)) = check(last, symbol_idx, &mut seen_numbers) {
                        adj_num.push(num);
                        // println!("Adding ({}, {}) / {}", last.line_number, id, num);
                    }
                    if let Some((id, num)) = check(last, symbol_idx + 1, &mut seen_numbers) {
                        adj_num.push(num);
                        // println!("Adding ({}, {}) / {}", last.line_number, id, num);
                    }
                }

                let mut seen_numbers: HashMap<u32, bool> = HashMap::new();
                // handle left + right
                if let Some((id, num)) = check(cur, symbol_idx - 1, &mut seen_numbers) {
                    adj_num.push(num);
                    // println!("Adding ({}, {}) / {}", cur.line_number, id, num);
                }
                if let Some((id, num)) = check(cur, symbol_idx + 1, &mut seen_numbers) {
                    adj_num.push(num);
                    // println!("Adding ({}, {}) / {}", cur.line_number, id, num);
                }

                if let Some(ref next) = next_lni {
                    let mut seen_numbers: HashMap<u32, bool> = HashMap::new();
                    if let Some((id, num)) = check(next, symbol_idx - 1, &mut seen_numbers) {
                        adj_num.push(num);
                        // println!("Adding ({}, {}) / {}", next.line_number, id, num);
                    }
                    if let Some((id, num)) = check(next, symbol_idx, &mut seen_numbers) {
                        adj_num.push(num);
                        // println!("Adding ({}, {}) / {}", next.line_number, id, num);
                    }
                    if let Some((id, num)) = check(next, symbol_idx + 1, &mut seen_numbers) {
                        adj_num.push(num);
                        // println!("Adding ({}, {}) / {}", next.line_number, id, num);
                    }
                }
            
                if adj_num.len() == 2 {
                    accum += (adj_num[0] * adj_num[1]);
                }
            }
        }

        line_no = line_no + 1;
        last_lni = cur_lni;
        cur_lni = next_lni;
    }

    println!("accum: {}", accum);
}


fn part1() {
    let sample_input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let sample_input2 = r".....
1*4..
..1..
.....
.....
..10.
1+...
";

    let real_input = std::fs::read_to_string("input.txt").expect("contents");

    let re = &Regex::new(r"([0-9]+)").expect("a regex");
    let mut line_no: u32 = 0;

    let mut last_lni: Option<LineInfo> = None;
    let mut cur_lni: Option<LineInfo> = None;
    let mut next_lni: Option<LineInfo> = None;

    let mut accum = 0;

    for l in sample_input.split("\n") {
    // for l in sample_input2.split("\n") {
    // for l in real_input.split("\n") {
        // println!("\n----------------------\n");
        next_lni = Some(LineInfo::from_str(re, line_no, l));

        // println!("last: {:?}\ncur:  {:?}\nnext: {:?}",
        //     last_lni.as_ref().map(|lni| lni.line_number),
        //     cur_lni.as_ref().map(|lni| lni.line_number),
        //     next_lni.as_ref().map(|lni| lni.line_number),
        // );

        if let Some(ref cur) = cur_lni {
            // println!("{:?}", cur);
            for symbol_idx_idx in 0..cur.symbols.len() {
                let symbol_idx = cur.symbols[symbol_idx_idx];
                // println!("processing: {}", cur.symbols[symbol_idx_idx]);

                if let Some(ref last) = last_lni {
                    let mut seen_numbers: HashMap<u32, bool> = HashMap::new();

                    if let Some((id, num)) = check(last, symbol_idx - 1, &mut seen_numbers) {
                        accum += num;
                        // println!("Adding ({}, {}) / {}", last.line_number, id, num);
                    }
                    if let Some((id, num)) = check(last, symbol_idx, &mut seen_numbers) {
                        accum += num;
                        // println!("Adding ({}, {}) / {}", last.line_number, id, num);
                    }
                    if let Some((id, num)) = check(last, symbol_idx + 1, &mut seen_numbers) {
                        accum += num;
                        // println!("Adding ({}, {}) / {}", last.line_number, id, num);
                    }
                }

                let mut seen_numbers: HashMap<u32, bool> = HashMap::new();
                // handle left + right
                if let Some((id, num)) = check(cur, symbol_idx - 1, &mut seen_numbers) {
                    accum += num;
                    // println!("Adding ({}, {}) / {}", cur.line_number, id, num);
                }
                if let Some((id, num)) = check(cur, symbol_idx + 1, &mut seen_numbers) {
                    accum += num;
                    // println!("Adding ({}, {}) / {}", cur.line_number, id, num);
                }

                if let Some(ref next) = next_lni {
                    let mut seen_numbers: HashMap<u32, bool> = HashMap::new();
                    if let Some((id, num)) = check(next, symbol_idx - 1, &mut seen_numbers) {
                        accum += num;
                        // println!("Adding ({}, {}) / {}", next.line_number, id, num);
                    }
                    if let Some((id, num)) = check(next, symbol_idx, &mut seen_numbers) {
                        accum += num;
                        // println!("Adding ({}, {}) / {}", next.line_number, id, num);
                    }
                    if let Some((id, num)) = check(next, symbol_idx + 1, &mut seen_numbers) {
                        accum += num;
                        // println!("Adding ({}, {}) / {}", next.line_number, id, num);
                    }
                }
            }
        }

        line_no = line_no + 1;
        last_lni = cur_lni;
        cur_lni = next_lni;
    }

    println!("accum: {}", accum);
}

fn main() {
    part2();
}
