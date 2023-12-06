
#[derive(Debug)]
struct Mapping {
    dst: u64,
    src: u64,
    cnt: u64,
}

#[derive(Debug)]
struct Maps {
    seeds: Vec<u64>,
    maps: Vec<Vec<Mapping>>,
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u64 {
    let maps = parseInput(input);
    let mut minLocation = u64::MAX;

    for x in (0..maps.seeds.len()).step_by(2) {
        let start = maps.seeds[x];
        let count = maps.seeds[x+1];
        let mut seed = start;

        while seed < start + count {
            let mut minNext = u64::MAX;
            let mut location = seed;

            for map in &maps.maps {
                location = followMapping(location, &map, &mut minNext);
            }

            minLocation = std::cmp::min(minLocation, location);
            seed += minNext;
        }
    }

    minLocation
}

fn followMapping(seed: u64, mappings: &Vec<Mapping>, minNext: &mut u64) -> u64 {

    for mapping in mappings {
        if seed >= mapping.src && seed < mapping.src + mapping.cnt {
            *minNext = std::cmp::min(*minNext, mapping.src + mapping.cnt - seed);
            return mapping.dst + (seed - mapping.src)
        } else if seed < mapping.src {
            *minNext = std::cmp::min(*minNext, mapping.src - seed)
        }
    }

    seed
}

fn parseInput(input: &str) -> Maps {
    let mut lines:Vec<&str> = input.lines().collect();
    lines.insert(lines.len(), "");
    let mut seeds: Vec<u64> = Default::default();
    let mut maps: Vec<Vec<Mapping>> = Default::default();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("seeds:") {
            seeds = line.split_once(":").unwrap().1.split_whitespace().into_iter().map(|x| x.parse().unwrap()).collect();
        } else if line.contains("map") {
            maps.push(parseMap(&mut i, &lines));
        }
        i += 1;
    }

    Maps {seeds, maps}
}

fn parseMap(i: &mut usize, lines: &Vec<&str>) -> Vec<Mapping> {
    let mut mappings: Vec<Mapping> = Default::default();
    *i += 1;
    let mut line = lines[*i];
    while line.len() > 0 {
        mappings.push(parseMapping(line));
        *i += 1;
        line = lines[*i];
    }

    mappings
}

fn parseMapping(line: &str) -> Mapping {

    let nums:Vec<u64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let dst = nums[0];
    let src = nums[1];
    let cnt = nums[2];

    Mapping {dst, src, cnt}
}
