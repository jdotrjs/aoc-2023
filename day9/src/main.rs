use std::collections::HashMap;
use std::env;

/*
To do this, start by making a new sequence from the difference at each step of
your history. If that sequence is not all zeroes, repeat this process, using
the sequence you just generated as the input sequence. Once all of the values
in your latest sequence are zeroes, you can extrapolate what the next value of
the original history should be.
 */

fn num_str_to_vec(input: &str) -> Vec<i64> {
    let mut r = Vec::new();
    for e in input.split(" ") {
        if e.len() == 0 {
            continue;
        }
        r.push(e.trim().parse().expect("a number"));
    }
    return r;
}

fn perform_diffs(input: &Vec<i64>) -> Vec<i64> {
    let mut diffs = vec![];
    for idx in 0..(input.len() - 1) {
        diffs.push(input[idx+1] - input[idx]);
    }
    return diffs;
}

fn perform_diffs_2(input: &Vec<i64>) -> Vec<i64> {
    let mut diffs = vec![];
    for idx_offset in 1..(input.len()) {
        let idx = input.len() - idx_offset;
        diffs.push(input[idx] - input[idx-1]);
    }
    return diffs;
}

fn final_step(input: &Vec<i64>) -> bool {
    input.iter().all(|x| *x == 0)
}

fn recurse(cur: &mut Vec<i64>) {
    if final_step(cur) {
        cur.push(0);
    } else {
        let mut next = perform_diffs(cur);
        recurse(&mut next);
        let last_cur = cur.last().unwrap();
        let last_next = next.last().unwrap();
        cur.push(*last_cur + *last_next);
    }
}

fn recurse_2(cur: &mut Vec<i64>) {
    if final_step(cur) {
        cur.push(0);
    } else {
        let mut next = perform_diffs(cur);
        recurse_2(&mut next);
        let first_cur = cur.first().unwrap();
        let first_next = next.first().unwrap(); 
        cur.insert(0, first_cur - first_next);
    }
}

fn part1(input: &String) {
    let mut acc = 0;
    for l in input.lines() {
        let mut readings = num_str_to_vec(l);
        recurse(&mut readings);
        acc += readings.last().unwrap();
    }

    println!("acc: {acc}");
}

fn part2(input: &String) {
    let mut acc = 0;
    for l in input.lines() {
        let mut readings = num_str_to_vec(l);
        recurse_2(&mut readings);
        acc += readings.first().unwrap();
    }

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