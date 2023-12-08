use std::collections::HashMap;
use std::env;
use std::str::Lines;

#[derive(Debug)]
enum Direction{
    Left,
    Right,
}

impl Direction {
    fn parse_path(input: &String) -> Vec<Direction> {
        let mut path = vec![];
        for c in input.as_bytes() {
            match *c {
                b'L' => { path.push(Direction::Left); },
                b'R' => { path.push(Direction::Right); },
                _ => { panic!("LR only"); },
            }
        }
        return path;
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn from_str(input: &String) -> Node {
        let mut parts = input.split("=");
        let name = parts.nth(0).unwrap().trim();
        let rest = parts.nth(0).unwrap();
        let x = rest.replace("(", "");
        let y = x.replace(")", "");
        let mut next = y.split(",");
        let left = next.nth(0).unwrap().trim();
        let right = next.nth(0).unwrap().trim();

        // println!("{name} -> {left}, {right}");

        Node {
            name: String::from(name),
            left: String::from(left),
            right: String::from(right),
        }
    }

    fn parse_map(input: Lines<'_>) -> HashMap<String, Node> {
        let mut node_map = HashMap::new();
        for l in input {
            let line = l.trim();
            if line.len() == 0 { continue; }
            
            let n = Node::from_str(&String::from(line));
            node_map.insert(n.name.clone(), n);
        }
        return node_map;
    }

    fn start_node(&self) -> bool {
        self.name.ends_with("A")
    }

    fn stop_node(&self) -> bool {
        self.name.ends_with("Z")
    }
}

fn part1(input: &String, start: &String, end: &dyn Fn(&String) -> bool) {
    let mut lines = input.lines();
    let path_line = lines.nth(0).unwrap();
    let path = Direction::parse_path(&path_line.to_string());
    // println!("{:?}", path);
    let node_map = Node::parse_map(lines);
    let steps = common(&node_map, &path, start, end);
    println!("steps: {steps}");
}

fn common<F>(node_map: &HashMap<String, Node>, path: &Vec<Direction>, start: &String, end: F) -> usize
where
    F: Fn(&String) -> bool,
{
    // println!("{:?}", node_map);

    let mut steps = 0;

    let mut cur = node_map.get(start).unwrap();
    while !end(&cur.name) {
        // println!("cur: {}", &cur.name);
        let next_dir = path.get(steps % path.len()).unwrap();
        match next_dir {
            Direction::Left => {
                // println!("Left");
                cur = &node_map[&cur.left];
            },
            Direction::Right => {
                // println!("Right");
                cur = &node_map[&cur.right];
            }
        }
        steps = steps + 1;
    }

    // println!("steps: {steps}");
    return steps;
}

fn all_stop(nodes: &Vec<&Node>) -> bool {
    nodes.iter().all(|v| v.stop_node())
}

fn part2(input: &String) {
    let mut lines = input.lines();
    let path_line = lines.nth(0).unwrap();
    let path = Direction::parse_path(&path_line.to_string());
    let node_map = Node::parse_map(lines);

    let start_nodes: Vec<&String> = node_map.values()
        .filter(|v| v.start_node())
        .map(|v| &v.name)
        .collect();

    let mut cur_nodes: Vec<&Node> = vec![];
    for sn in start_nodes.iter() {
        cur_nodes.push(node_map.get(*sn).unwrap());
    }

    for cn in cur_nodes {
        let cn_steps = common(&node_map, &path, &cn.name, |x| x.ends_with("Z"));
        println!("{} -> {}", &cn.name, cn_steps);
    }

    println!("find LCM for the above");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_path = String::from("test.txt");
    let fp = args.get(1).unwrap_or(&default_path);

    let input = std::fs::read_to_string(fp).expect("input");
    part1(&input, &String::from("AAA"), &|x| x == "ZZZ");
    part2(&input);
}
