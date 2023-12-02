use std::collections::HashMap;

use regex::Regex;

fn print_draw_map(m: &HashMap<&str, u32>) {
    for (k, v) in m.iter() {
        println!("  {} -> {}", k, v);
    }
}
fn process_draws(input: &str) -> Vec<HashMap<&str, u32>> {
    let mut draws = vec![];

    for draw in input.split(";") {
        let mut draw_map = HashMap::new();
        
        for pair in draw.split(",") {
            let mut p = pair.trim().split(" ");
            let count = p.next().expect("a count");
            let color = p.next().expect("a color");
            draw_map.insert(color, count.parse().expect("a number"));
        }

        draws.push(draw_map);
    }

    return draws;
}

// game in -> power out
fn part2(re: &Regex, input: &str) -> u32 {
    // println!("\n{}", input);
    if let Some(caps) = re.captures(input) {
        let id_match_opt = caps.name("id");
        let draws_match_opt = caps.name("draws");
    
        let (_id, draws) = match (id_match_opt, draws_match_opt) {
            (Some(id), Some(draws)) => (
                id.as_str(),
                draws.as_str()
            ),
            _ => return 0,
        };

        let game_draws = process_draws(draws);
        
        let mut max_draws: HashMap<&str, u32> = HashMap::new();

        for draw_map in game_draws {
            for (k, v) in draw_map.iter() {
                if let Some(cur_max) = max_draws.get(k) {
                    if cur_max < v {
                        max_draws.insert(k, *v);
                    }
                } else {
                    max_draws.insert(k, *v);
                }
            }
        }

        // print_draw_map(&max_draws);

        let mut a = 1;
        for v in max_draws.values() {
            a *= *v;
        }
        return a;
    }

    return 0;
}

fn part1(re: &Regex, input: &str, game_bag: &HashMap<&str, u32>) -> (u32, bool) {
    if let Some(caps) = re.captures(input) {
        let id_match_opt = caps.name("id");
        let draws_match_opt = caps.name("draws");
    
        let (id, draws) = match (id_match_opt, draws_match_opt) {
            (Some(id), Some(draws)) => (
                id.as_str(),
                draws.as_str()
            ),
            _ => return (0, false)
        };

        let game_id: u32 = id.parse().expect("number");
        // let max_seen_per_game = HashMap::new();

        // println!("Game Id: {}", game_id);

        let game_draws = process_draws(draws);
        
        for draw_map in game_draws {
            // print_draw_map(&draw_map);
            for key in draw_map.keys() {
                if let Some(bag_count) = game_bag.get(key) {
                    if bag_count < draw_map.get(key).unwrap() {
                        return (game_id, false);
                    }
                } else {
                    return (game_id, false);
                }
            }
        }
        return (game_id, true);
    }
    return (0, false);
}

fn main() {
    let sample_input = vec![
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ];

    let real_input_str =
        std::fs::read_to_string("input.txt")
        .expect("contents");
    let real_input = real_input_str.split("\n");

    let mut bag: HashMap<&str, u32> = HashMap::new();
    bag.insert("red", 12);
    bag.insert("green", 13);
    bag.insert("blue", 14);

    println!("Game Bag:");
    print_draw_map(&bag);

    let mut sum = 0;
    let mut sum2 = 0;

    let re = Regex::new(r"Game (?<id>\d+): (?<draws>.*)").expect("a regex");

    for line in real_input {
        let (game_id, possible) = part1(&re, line, &bag);
        if possible {
            sum += game_id;
        }
        sum2 += part2(&re, line);
    }

    println!("part1: {}", sum);
    println!("part2: {}", sum2)
}
