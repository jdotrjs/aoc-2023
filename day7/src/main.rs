use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::ops::Index;

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_str(input: &String, jokers_wild: bool) -> HandType {
        // println!("input: {}", input);
        if input == "JJJJJ" {
            return HandType::FiveOfAKind;
        }
        // card -> count
        let mut m: HashMap<char, i32> = HashMap::new();

        // holds current strongest card
        let mut strongest_card = ('A', -1);

        for card in input.as_bytes() {
            let card_c = char::from(*card);
            if m.contains_key(&card_c) {
                 m.insert(card_c, m.get(&card_c).unwrap() + 1);
            } else {
                m.insert(card_c, 1);
            }

            if let Some(card_count) = m.get(&card_c) {
                if *card_count >= strongest_card.1 && card_c != 'J' {
                    strongest_card = (card_c, *card_count);
                }
            }
        }

        if jokers_wild {
            // reassign jokers to the strongest card
            if let Some(joker_count) = m.get(&'J') {
                let cur = m.get(&strongest_card.0).unwrap();
                m.insert(strongest_card.0, *cur + joker_count);
                // not strictly required but do it anyway
                m.remove(&'J');
            }
        }

        // println!("m: {:?}", m);

        // count -> counts
        let mut card_counts = HashMap::new();
        for count in 0..6 {
            for key in m.keys() {
                // jokers will be a special case
                if jokers_wild && *key == 'J' {
                    continue;
                }
                let key_count = *m.get(key).unwrap();
                if key_count == count {
                    if card_counts.contains_key(&count) {
                        let cur = *card_counts.get(&count).unwrap();
                        card_counts.insert(count, cur + 1);
                    } else {
                        card_counts.insert(count, 1);
                    }
                }
            }
        }

        // println!("card_counts: {:?}", card_counts);

        if *card_counts.get(&5).unwrap_or(&0) == 1 {
            return HandType::FiveOfAKind;
        }
        if *card_counts.get(&4).unwrap_or(&0) == 1 {
            return HandType::FourOfAKind;
        }
        if *card_counts.get(&3).unwrap_or(&0) == 1 {
            if *card_counts.get(&2).unwrap_or(&0) == 1 {
                return HandType::FullHouse;
            }
            if *card_counts.get(&1).unwrap_or(&0) == 2 {
                return HandType::ThreeOfAKind;
            }
        }
        if *card_counts.get(&2).unwrap_or(&0) == 2 {
            return HandType::TwoPair;
        }
        if *card_counts.get(&2).unwrap_or(&0) == 1 {
            return HandType::OnePair;
        }
        if *card_counts.get(&1).unwrap_or(&0) == 5 {
            return HandType::HighCard;
        }

        panic!("Shouldn't be here: {}", input);
    }

    fn points(&self) -> u32 {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

#[derive(Debug)]
struct Hand {
    typ: HandType,
    wager: u32,
    cards: Vec<char>,
}

impl Hand {
    fn from_str(input: &String, jokers_wild: bool) -> Hand {
        let mut parts = input.split(" ");
        let cards = parts.next().unwrap();
        let wager = parts.next().unwrap().parse().expect("number");
        let mut card_vec: Vec<char> = cards.chars().collect();
        // card_vec.sort_by(|a, b| card_points(*b).cmp(&card_points(*a)));

        Hand {
            typ: HandType::from_str(&String::from(cards), jokers_wild),
            wager,
            cards: card_vec,
        }
    }
}

fn card_points(input: char) -> u32 {
    let cards = "AKQT98765432J";
    let max_value = cards.len();
    let r = cards.find(|x| x == input).unwrap();
    return (max_value - r) as u32;
}

fn part1(input: &String) {
    // println!("{}\n\n", input);
    let mut hands: Vec<Hand> = input.lines()
        .map(|line| line.trim())
        .filter(|line| line.len() != 0)
        .map(|line| Hand::from_str(&line.to_string(), false))
        .collect();

    hands.sort_by(|a, b| {
        let a_points = a.typ.points();
        let b_points = b.typ.points();
        if a_points != b_points {
            // return b_points.cmp(&a_points);
            return a_points.cmp(&b_points);
        } else {
            let a_hand = &a.cards;
            let b_hand = &b.cards;
            for card_idx in 0..5 {
                let a_card = a_hand.index(card_idx);
                let b_card = b_hand.index(card_idx);
                if *a_card != *b_card {
                    return card_points(*a_card).cmp(&card_points(*b_card));
                }
            }
        }
        return Ordering::Equal;
    });

    let mut acc = 0;
    for rank in 0..hands.len() {
        // println!("{} * {}", rank+1, hands.index(rank).wager);
        acc = acc + (
            ((rank as u32)+1) * hands.index(rank).wager);
    }

    println!("acc: {acc}");
}

fn part2(input: &String) {
    let mut hands: Vec<Hand> = input.lines()
        .map(|line| line.trim())
        .filter(|line| line.len() != 0)
        .map(|line| Hand::from_str(&line.to_string(), true))
        .collect();

    hands.sort_by(|a, b| {
        let a_points = a.typ.points();
        let b_points = b.typ.points();
        if a_points != b_points {
            // return b_points.cmp(&a_points);
            return a_points.cmp(&b_points);
        } else {
            let a_hand = &a.cards;
            let b_hand = &b.cards;
            for card_idx in 0..5 {
                let a_card = a_hand.index(card_idx);
                let b_card = b_hand.index(card_idx);
                if *a_card != *b_card {
                    return card_points(*a_card).cmp(&card_points(*b_card));
                }
            }
        }
        return Ordering::Equal;
    });

    let mut acc = 0;
    for rank in 0..hands.len() {
        acc = acc + (
            ((rank as u32)+1) * hands.index(rank).wager);
    }

    println!("acc: {acc}");
}

fn main() {
    // println!("A -> {}", card_points('A'));
    // println!("K -> {}", card_points('K'));
    // println!("2 -> {}", card_points('2'));
    // println!("J -> {}", card_points(b'J'));
    // println!("T -> {}", card_points(b'T'));

    let args: Vec<String> = env::args().collect();
    let default_path = String::from("test.txt");
    let fp = args.get(1).unwrap_or(&default_path);

    let input = std::fs::read_to_string(fp).expect("input");
    part1(&input);
    part2(&input);
}
