use std::env;
use regex::Regex;
use once_cell::sync::Lazy;
use std::collections::HashMap;

const CARD_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"Card\s+(?<game_no>[0-9]+): (?<winning>[0-9 ]+)\|(?<have>[0-9 ]+)").expect("bad regex"));

#[derive(Debug)]
struct Card {
    card_id: u32,
    winning_numbers: Vec<u32>,
    listed_numbers: Vec<u32>,
    match_count: u32,
}

fn num_str_to_vec(input: &str) -> Vec<u32> {
    let mut r = Vec::new();
    for e in input.split(" ") {
        if e.len() == 0 {
            continue;
        }
        r.push(e.trim().parse().expect("a number"));
    }
    return r;
}

impl Card {
    fn points(&self) -> u32 {
            if self.match_count == 0 {
                return 0;
            }
            return u32::pow(2, self.match_count - 1);
    }

    fn from_str(input: &str) -> Option<Card> {
        let cap_opt = CARD_RE.captures(input);
        if cap_opt.is_none() {
            return None;
        }

        let cap = cap_opt.unwrap();

        let card_id = cap.name("game_no").unwrap().as_str().parse().expect("card number");
        let winning_numbers_str = cap.name("winning").unwrap().as_str();
        let listed_numbers_str = cap.name("have").unwrap().as_str();

        let winning_numbers = num_str_to_vec(winning_numbers_str);
        let listed_numbers = num_str_to_vec(listed_numbers_str);

        let mut match_count: u32 = 0;
        for winner in winning_numbers.as_slice() {
            if listed_numbers.contains(&winner) {
                match_count = match_count + 1;
            }
        }
        match_count = match_count;


        // println!("id: {card_id}\nwinning: {winning_numbers_str}, listed: {listed_numbers_str}");

        Some(Card{
            card_id,
            winning_numbers,
            listed_numbers,
            match_count,
        })
    }
}

fn part1(input: &String) {
    let accum = 0;
    // println!("{}", input);

    let mut accum = 0;
    
    for line in input.lines() {
        let c_opt = Card::from_str(line);
        // println!("c: {:?}", c_opt);
        if let Some(ref c) = c_opt {
            accum += c.points();
        }

    }

    println!("accum: {accum}");
}

fn part2(input: &String) {
    let mut card_catalog = HashMap::new();
    let mut to_evaluate = vec![];
    let mut max_card_id = 0;

    for line in input.lines() {
        let c_opt = Card::from_str(line);
        if let Some(c) = c_opt {
            max_card_id = u32::max(max_card_id, c.card_id);
            to_evaluate.push(c.card_id);
            card_catalog.insert(c.card_id, c);
        }

    }

    let mut accumulated_cards: u32 = 0;

    while to_evaluate.len() > 0 {
        let next = to_evaluate.pop().unwrap();
        let c = card_catalog.get(&next).unwrap();
        if c.match_count > 0 {
            accumulated_cards += c.match_count;
            let dupe_start = c.card_id + 1;
            let dupe_end = u32::min(max_card_id, dupe_start + c.match_count);
            for dupe_id in dupe_start..dupe_end {
                to_evaluate.push(dupe_id);
            }
        }
    }

    println!("initial cards: {}", card_catalog.len());
    println!("accumulated: {}", accumulated_cards);
    println!("total cards: {}", accumulated_cards + (card_catalog.len() as u32));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_path = String::from("test.txt");
    let fp = args.get(1).unwrap_or(&default_path);

    let input = std::fs::read_to_string(fp).expect("input");
    part2(&input);
}
