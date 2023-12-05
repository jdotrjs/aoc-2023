use std::env;
use regex::Regex;
use once_cell::sync::Lazy;
// use std::collections::HashMap;

const MAP_TYPE_RE: Lazy<Regex> = Lazy::new(||
    Regex::new(r"(?<map_type>.*)\s+map:").expect("bad regex")
);

// Each line within a map contains three numbers:
//      the destination range start
//      the source range start
//      and the range length.
const MAP_MAPPING_RE: Lazy<Regex> = Lazy::new(||
    Regex::new(r"(?<dest_start>[0-9]+)\s+(?<src_start>[0-9]+)\s+(?<range_len>[0-9]+)").expect("bad regex")
);

const SEEDS_RE: Lazy<Regex> = Lazy::new(||
    Regex::new(r"seeds: (?<seeds>.*)").expect("bad regex")
);

#[derive(Debug)]
struct Mapper {
    mappings: Vec<(u64, u64, u64)>
}

impl Mapper {
    fn nil() -> Mapper {
        Mapper{mappings: vec![]}
    }

    fn from_vec(input: Vec<(u64, u64, u64)>) -> Mapper {
        Mapper{mappings: input}
    }

    fn map(&self, from: u64) -> u64 {
        for idx in 0..self.mappings.len() {
            let e = self.mappings.get(idx).unwrap();
            // println!("{:?}", e);
            
            let (src, dest, rng) = e;
            
            if from < *src {
                // println!("< src");
                continue;
            }
            let src_max = *src + *rng;
            if from < src_max {
                let tgt = *dest + (from - *src);
                // println!("maps to {tgt}");
                // TODO: seems likely to cause off-by-one
                return tgt;
            }
        }
        return from;
    }
}

#[derive(Debug)]
struct Almanac {
    to_plant: Vec<u64>,
    seed_soil_map: Vec<(u64, u64, u64)>,
    ss_mapper: Mapper,
    
    soil_fertilizer_map: Vec<(u64, u64, u64)>,
    sf_mapper: Mapper,
    
    fertilizer_water_map: Vec<(u64, u64, u64)>,
    fw_mapper: Mapper,
    
    water_light_map: Vec<(u64, u64, u64)>,
    wl_mapper: Mapper,
    
    light_temp_map: Vec<(u64, u64, u64)>,
    lt_mapper: Mapper,

    temp_humidity_map: Vec<(u64, u64, u64)>,
    th_mapper: Mapper,

    humidity_location_map: Vec<(u64, u64, u64)>,
    hl_mapper: Mapper,
}

struct Seed {
    id: u64
}

impl Seed {
    fn trace(&self, a: &Almanac) -> u64 {
        let seed = self.id;
        // println!("seed: {seed}");
        // println!("soil");
        let soil = a.ss_mapper.map(seed);
        // println!("fert");
        let fert = a.sf_mapper.map(soil);
        // println!("water");
        let wat = a.fw_mapper.map(fert);
        // println!("light");
        let light = a.wl_mapper.map(wat);
        // println!("temp");
        let temp = a.lt_mapper.map(light);
        // println!("humidity");
        let hum = a.th_mapper.map(temp);
        // println!("location");
        let loc = a.hl_mapper.map(hum);

        // println!("{seed} -> {soil} -> {fert} -> {wat} -> {light} -> {temp} -> {hum} -> {loc}");

        return loc;
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

impl Almanac {
    fn from_str(input: &String) -> Almanac {
        let mut cur_map_type = "";
        let mut cur_map: Vec<(u64, u64, u64)> = Vec::new();
        let mut almanac = Almanac{
            to_plant: vec![],
            seed_soil_map: vec![],
            ss_mapper: Mapper::nil(),
            soil_fertilizer_map: vec![],
            sf_mapper: Mapper::nil(),
            fertilizer_water_map: vec![],
            fw_mapper: Mapper::nil(),
            water_light_map: vec![],
            wl_mapper: Mapper::nil(),
            light_temp_map: vec![],
            lt_mapper: Mapper::nil(),
            temp_humidity_map: vec![],
            th_mapper: Mapper::nil(),
            humidity_location_map: vec![],
            hl_mapper: Mapper::nil(),
        };

        for line_raw in input.lines() {
            let line = line_raw.trim();
            if line.len() == 0 {
                continue;
            }

            // println!("l: {}", line);

            if SEEDS_RE.is_match(line) {
                let seeds = SEEDS_RE.captures(line).unwrap().name("seeds").unwrap().as_str();
                almanac.to_plant = num_str_to_vec(seeds);
                continue;
            }


            if MAP_TYPE_RE.is_match(line) {
                match cur_map_type {
                    "seed-to-soil" => {
                        almanac.ss_mapper = Mapper::from_vec(cur_map.clone());
                        almanac.seed_soil_map.append(&mut cur_map);
                    },
                    "soil-to-fertilizer" => {
                        almanac.sf_mapper = Mapper::from_vec(cur_map.clone());
                        almanac.soil_fertilizer_map.append(&mut cur_map);
                    },
                    "fertilizer-to-water" => {
                        almanac.fw_mapper = Mapper::from_vec(cur_map.clone());
                        almanac.fertilizer_water_map.append(&mut cur_map);
                    },
                    "water-to-light" => {
                        almanac.wl_mapper = Mapper::from_vec(cur_map.clone());
                        almanac.water_light_map.append(&mut cur_map);
                    },
                    "light-to-temperature" => {
                        almanac.lt_mapper = Mapper::from_vec(cur_map.clone());
                        almanac.light_temp_map.append(&mut cur_map);
                    },
                    "temperature-to-humidity" => {
                        almanac.th_mapper = Mapper::from_vec(cur_map.clone());
                        almanac.temp_humidity_map.append(&mut cur_map);
                    },
                    "humidity-to-location" => {
                        almanac.hl_mapper = Mapper::from_vec(cur_map.clone());
                        almanac.humidity_location_map.append(&mut cur_map);
                    },
                    "" => (), // noop on first map declaration
                    _ => { panic!("bullshit map type: {}", cur_map_type); }
                }
                cur_map = Vec::new();
                cur_map_type = MAP_TYPE_RE.captures(line).unwrap().name("map_type").unwrap().as_str();
                continue;
            }

            if MAP_MAPPING_RE.is_match(line) {
                let captures = MAP_MAPPING_RE.captures(line).unwrap();
                let src = captures.name("src_start").unwrap().as_str().parse().expect("number");
                let dest = captures.name("dest_start").unwrap().as_str().parse().expect("number");
                let rng = captures.name("range_len").unwrap().as_str().parse().expect("number");
                // println!("line: {line}\nsrc: {src}\ndest: {dest}\nrng: {rng}");
                cur_map.push((src, dest, rng));
            }
        }

        if cur_map_type == "humidity-to-location" {
            almanac.hl_mapper = Mapper::from_vec(cur_map.clone());
            almanac.humidity_location_map.append(&mut cur_map);
        }

        almanac
    }
}

fn part1(input: &String) {
    let a = Almanac::from_str(input);
    // println!("{:?}", a);

    println!("walking seeds");

    let mut min = None;

    for idx in 0..a.to_plant.len() {
        let id = a.to_plant.get(idx).unwrap();
        let sd = Seed{id: *id};
        let loc = sd.trace(&a);
        println!("{} -> {}", *id, loc);
        if min.is_none() {
            min = Some(loc)
        } else {
            min = Some(u64::min(min.unwrap(), loc));
        }
    }
    println!("part1: {}", min.unwrap());
}

fn part2(input: &String) {
    let a = Almanac::from_str(input);
    let mut seeds = Vec::new();
    let mut idx = 0;
    while idx < a.to_plant.len() {
        seeds.push((*a.to_plant.get(idx).unwrap(), *a.to_plant.get(idx+1).unwrap()));
        idx += 2;
    }
    
    // println!("{:?}", a);

    println!("walking seeds");

    let mut min = None;

    for (range_start, range) in seeds.iter() {
        let range_stop = *range_start + *range;
        for seed in *range_start..range_stop {
        let sd = Seed{id: seed};
        let loc = sd.trace(&a);
        // println!("{} -> {}", seed, loc);
        if min.is_none() {
            min = Some(loc)
        } else {
            min = Some(u64::min(min.unwrap(), loc));
        }
        }
    }
    println!("part2: {}", min.unwrap());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_path = String::from("test.txt");
    let fp = args.get(1).unwrap_or(&default_path);

    let input = std::fs::read_to_string(fp).expect("input");
    part2(&input);
}
