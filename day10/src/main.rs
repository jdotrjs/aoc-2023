use std::collections::{HashMap, HashSet};
use std::env;
use std::ops::Index;
use colored::Colorize;

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

/*

    | is a vertical pipe connecting north and south.
    - is a horizontal pipe connecting east and west.
    L is a 90-degree bend connecting north and east.
    J is a 90-degree bend connecting north and west.
    7 is a 90-degree bend connecting south and west.
    F is a 90-degree bend connecting south and east.
    . is ground; there is no pipe in this tile.
    S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

*/

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn xy_delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }

    fn from_delta(xy: (i32, i32)) -> Direction {
        match xy {
            (0, -1) => Direction::North,
            (0, 1) => Direction::South,
            (1, 0) => Direction::East,
            (-1, 0) => Direction::West,
            _ => panic!("bad xy: {:?}", xy)
        }
    }

    fn inverted(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn heading(from: (i32, i32), to: (i32, i32)) -> Direction {
        Direction::from_delta((to.0 - from.0, to.1 - from.1))
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Piece {
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
    Start,
    None
}

impl Piece {
    fn from_char(c: &char) -> Piece {
        match c {
            '|' => Piece::Vertical,
            '-' => Piece::Horizontal,
            'L' => Piece::BendNE,
            'J' => Piece::BendNW,
            '7' => Piece::BendSW,
            'F' => Piece::BendSE,
            'S' => Piece::Start,
            '.' => Piece::None,
            _ => panic!("lolwtf '{}'", c),
        }
    }

    fn connections(&self) -> Option<Vec<Direction>> {
        match self {
            Piece::Vertical => Some(vec![Direction::North,Direction::South]),
            Piece::Horizontal => Some(vec![Direction::West,Direction::East]),
            Piece::BendNE => Some(vec![Direction::North,Direction::East]),
            Piece::BendNW => Some(vec![Direction::North,Direction::West]),
            Piece::BendSW => Some(vec![Direction::South,Direction::West]),
            Piece::BendSE => Some(vec![Direction::South,Direction::East]),
            Piece::Start => None,
            Piece::None => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Piece::Vertical => "┃",
            Piece::Horizontal => "━",
            Piece::BendNE => "┗",
            Piece::BendNW => "┛",
            Piece::BendSW => "┓",
            Piece::BendSE => "┏",
            Piece::Start => "S",
            Piece::None => ".",
        }
    }

    fn is_start(&self) -> bool {
        match self {
            Piece::Start => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
struct Location {
    piece: Piece,
    x: i32,
    y: i32,
    is_exit: bool,
    synthetic: bool,
    on_path: bool,
}

impl Location {
    fn connects(&self, dir:&Direction, board: &HashMap<(i32, i32), Location>) -> bool {
        // a none-piece can't connect to anything
        if self.piece == Piece::None {
            return false;
        }

        let mut my_match = false;
        // a start can connect to anything
        if self.piece.is_start() {
            my_match = true;
        } else {
            // otherwise our outgoing directions have to match the desired
            // path
            if self.piece.connections().unwrap().contains(&dir) {
                // println!("{:?} contains {:?}", self.piece.connections(), dir);
                my_match = true;
            }
        }

        // if we can't match then we can't
        if !my_match {
            return false;
        }

        // get target location
        let xy_delta = dir.xy_delta();
        let tgt_xy = (self.x + xy_delta.0, self.y + xy_delta.1);
        let tgt_loc_opt = board.get(&tgt_xy);

        if tgt_loc_opt.is_none() {
            return false;
        }
        
        let tgt_loc = tgt_loc_opt.unwrap();
        let mut tgt_match = false;

        if let Some(connections) = tgt_loc.piece.connections() {
            tgt_match = connections.contains(&dir.inverted());
        }
        return tgt_match || tgt_loc.piece.is_start();
    }

    fn travel(&self, dir: &Direction) -> (i32, i32) {
        let xy_delta = dir.xy_delta();
        (self.x + xy_delta.0, self.y + xy_delta.1)
    }

    fn make_path(&mut self) {
        self.on_path = true;
    }

    fn make_exit(&mut self) {
        self.is_exit = true;
    }
}

fn parse_board(input: &String) -> ((i32, i32), HashMap<(i32, i32), Location>) {
    let mut board: HashMap<(i32, i32), Location> = HashMap::new();
    let mut y = 0;

    let mut start_loc = None;

    for l in input.lines() {
        let mut x = 0;
        for c in l.chars() {
            let loc = Location {
                piece: Piece::from_char(&c),
                x,
                y,
                is_exit: false,
                synthetic: false,
                on_path: false,
            };
            // print!("{:?}, ", loc);

            board.insert((x, y), loc);
            if board.get(&(x,y)).unwrap().piece.is_start() {
                start_loc = Some((x, y));
            }

            x = x + 1;
        }
        // println!();
        y = y + 1;
    }
    return (start_loc.unwrap(), board);
}

fn part1(start_loc: (i32, i32), board: &HashMap<(i32, i32), Location>) -> Vec<(i32, i32)> {
    let mut path = vec![];
    let mut stop = false;

    let mut cur = board.get(&start_loc).unwrap();
    path.push((cur.x, cur.y));
    let mut moved = &Direction::North;

    let mut distance = 0;

    // find first step
    let dir_options = &vec![Direction::North, Direction::South,Direction::East, Direction::West];
    
    // println!("start: {:?}", last);
    for dir in dir_options {
        // println!("Checking {:?}", dir);
        if cur.connects(&dir, &board) {
            // println!("Match {:?}, travel to {:?}", dir, cur.travel(&dir));
            cur = board.get(&cur.travel(&dir)).unwrap();
            moved = &dir;
            break;
        }
    }

    while !stop {
        path.push((cur.x, cur.y));
        // println!("moved: {:?}", moved);
        // println!("cur: {:?}", cur);
        for dir in dir_options {
            if *dir == moved.inverted() {
                // println!("not checking {:?}", dir);
                // don't backtrack
                continue;
            }
            if cur.connects(&dir, &board) {
                cur = board.get(&cur.travel(&dir)).unwrap();
                moved = dir;
                break;
            }
        }
        distance = distance+1;
        stop = cur.piece.is_start();
    }

    println!("distance: {:?}", distance);

    return path;

}

fn reform_board(board: &HashMap<(i32, i32), Location>, path: &Vec<(i32, i32)>) -> (HashMap<(i32, i32), Location>, Vec<(i32, i32)>) {
    let (max_x, max_y) = max_xy(board);

    // construct a totally synthetic board
    let mut new_board = HashMap::new();
    for x in -1..((2*max_x)+2) {
        for y in -1..((2 * max_y) + 2) {
            new_board.insert((x, y), Location {
                piece: Piece::None,
                x,
                y,
                is_exit: (x == -1) || (y == -1) || (x == 2*max_x+1) || (y == 2*max_y+1),
                synthetic: true,
                on_path: false,
            });
        }
    }

    for x in 0..max_x+1 {
        for y in 0..max_y+1 {
            let original = board.get(&(x, y)).unwrap();
            new_board.insert((2*x, 2*y), Location {
                piece: original.piece,
                x: 2 * x,
                y: 2 * y,
                is_exit: false,
                synthetic: false,
                on_path: original.on_path,
            });
        }
    }

    let (new_max_x, new_max_y) = max_xy(&new_board);
    for x in -1..new_max_x + 1 {
        for y in -1..new_max_y + 1 {
            let loc = new_board.get(&(x, y)).unwrap();
            // don't mess with exit or non-synthetic tiles
            if loc.is_exit || !loc.synthetic {
                continue;
            }
            // TODO: for now assume everything is on the loop

        }
    }

    let mut new_path = Vec::new();
    for e in path {
        new_path.push((e.0 * 2, e.1 * 2));
    }
    
    return (new_board, new_path);
}

fn min_xy(board: &HashMap<(i32, i32), Location>) -> (i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    for (kx, ky) in board.keys() {
        max_x = i32::min(max_x, *kx);
        max_y = i32::min(max_y, *ky);
    }
    return (max_x, max_y);
}

fn max_xy(board: &HashMap<(i32, i32), Location>) -> (i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    for (kx, ky) in board.keys() {
        max_x = i32::max(max_x, *kx);
        max_y = i32::max(max_y, *ky);
    }
    return (max_x, max_y);
}

fn print_board(board: &HashMap<(i32, i32), Location>, path: &Vec<(i32, i32)>) {
    let (min_x, min_y) = min_xy(board);
    let (max_x, max_y) = max_xy(board);

    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            let loc = board.get(&(x, y)).unwrap();
 
            if path.contains(&(x,y)) {
                print!("{}", loc.piece.to_str().green());
            } else {
                if loc.is_exit {
                    print!("{}", "x".cyan());
                } else {
                    if loc.synthetic {
                        print!("{}", loc.piece.to_str().blue());
                    } else {
                        print!("{}", loc.piece.to_str());
                    }
                }
            }
        }
        println!();
    }
}

fn part2(start_loc: (i32, i32), board: &HashMap<(i32, i32), Location>, path: &Vec<(i32, i32)>) {
    print_board(board, path);

    // println!("{:?}", path);

    let (mut reformed, reformed_path) = reform_board(board, path);

    let new_start_loc = (start_loc.0 * 2, start_loc.1 * 2);

    let mut path_index = 0;
    let original = path.index(path_index);
    path_index = path_index + 1;
    let next = path.index(path_index);

    
        
    // println!("original: {:?}", original);
    // println!("next:     {:?}", next);
    // println!("{} -> {}", path_index, path_index + 1);
    let mut next_dir = Direction::heading(*original, *next);
    // println!("delta:    {:?}", next_dir.xy_delta());
    // println!("next_dir: {:?}", next_dir);

    let mut cur = &reformed.get_mut(&new_start_loc).unwrap().clone();

    while true {
        let synthetic_xy = cur.travel(&next_dir);

        let snapshot_loc = reformed.get_mut(&synthetic_xy).unwrap().clone();;
        let updated_loc = Location {
            piece: match next_dir {
                Direction::North => Piece::Vertical,
                Direction::South => Piece::Vertical,
                Direction::East => Piece::Horizontal,
                Direction::West => Piece::Horizontal,
            },
            x: snapshot_loc.x,
            y: snapshot_loc.y,
            is_exit: snapshot_loc.is_exit,
            synthetic: true,
            on_path: true,
        };
        reformed.insert(synthetic_xy, updated_loc.clone());
        
        cur = reformed.get_mut(&updated_loc.travel(&next_dir)).unwrap();
        if cur.piece.is_start() {
            break;
        }

        // println!("{} -> {}", path_index, (path_index + 1) % path.len());
        next_dir = Direction::heading(*path.index(path_index), *path.index((path_index + 1) % path.len()));
        path_index = path_index + 1;
    }
    // print_board(&reformed, &reformed_path);

    flood_board(&mut reformed, &reformed_path);

    // print_board(&reformed, &reformed_path);

    let mut acc = 0;
    for v in reformed.values() {
        let v_xy = (v.x, v.y);
        if v.synthetic || v.is_exit {
            continue;
        }
        if reformed_path.contains(&v_xy) || v.on_path {
            continue;
        }
        acc = acc + 1;
        // println!("{:?}", v_xy);
    }
    println!("{:?}", acc);
}

fn flood_board(board: &mut HashMap<(i32, i32), Location>, path: &Vec<(i32, i32)>) {
    let mut eval = vec![(-1, -1)];
    let mut seen = HashSet::new();

    let dirs = vec![Direction::North, Direction::South, Direction::East, Direction::West];

    while eval.len() > 0 {
        let e = eval.pop().unwrap();
        if seen.contains(&e) {
            continue;
        }
        seen.insert(e);
        {
            board.get_mut(&e).unwrap().make_exit();
        }

        let cur_loc = board.get(&e).unwrap();
        for d in dirs.iter() {
            let new_xy = cur_loc.travel(d);
            if let Some(new_loc) = board.get(&new_xy) {
                let new_xy = (new_loc.x, new_loc.y);
                if !path.contains(&new_xy) && !(new_loc.synthetic && new_loc.on_path) {
                    eval.push(new_xy);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_path = String::from("test.txt");
    let fp = args.get(1).unwrap_or(&default_path);

    let input = std::fs::read_to_string(fp).expect("input");
    let (start_loc, mut board) = parse_board(&input);
    let path = part1(start_loc, &mut board);
    part2(start_loc, &mut board, &path);
}