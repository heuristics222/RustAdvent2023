#[derive(Debug)]
struct Mapping {
    dst: u64,
    src: u64,
    cnt: u64,
}

#[derive(Debug)]
struct Maps {
    seeds: Vec<u64>,
    sts: Vec<Mapping>,
    stf: Vec<Mapping>,
    ftw: Vec<Mapping>,
    wtl: Vec<Mapping>,
    ltt: Vec<Mapping>,
    tth: Vec<Mapping>,
    htl: Vec<Mapping>,
}

pub(crate) fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u64 {
    let maps = parseInput(input);
    let mut locations:Vec<u64> = Default::default();
    // println!("{:#?}", maps);

    for mut seed in maps.seeds {
        seed = followMapping(seed, &maps.sts);
        seed = followMapping(seed, &maps.stf);
        seed = followMapping(seed, &maps.ftw);
        seed = followMapping(seed, &maps.wtl);
        seed = followMapping(seed, &maps.ltt);
        seed = followMapping(seed, &maps.tth);
        seed = followMapping(seed, &maps.htl);

        locations.insert(locations.len(), seed);
    }

    *locations.iter().min().unwrap()
}

fn followMapping(seed: u64, mappings: &Vec<Mapping>) -> u64 {

    for mapping in mappings {
        if seed >= mapping.src && seed < mapping.src + mapping.cnt {
            return mapping.dst + (seed - mapping.src)
        }
    }

    seed
}

fn parseInput(input: &str) -> Maps {
    let mut lines:Vec<&str> = input.lines().collect();
    lines.insert(lines.len(), "");
    let mut seeds: Vec<u64> = Default::default();
    let mut sts: Vec<Mapping> = Default::default();
    let mut stf: Vec<Mapping> = Default::default();
    let mut ftw: Vec<Mapping> = Default::default();
    let mut wtl: Vec<Mapping> = Default::default();
    let mut ltt: Vec<Mapping> = Default::default();
    let mut tth: Vec<Mapping> = Default::default();
    let mut htl: Vec<Mapping> = Default::default();

    for mut i in 0..lines.len() {
        let mut line = lines[i];
        if line.starts_with("seeds:") {
            seeds = line.split_once(":").unwrap().1.split_whitespace().into_iter().map(|x| x.parse().unwrap()).collect();
        } else if line.starts_with("seed-to-soil map:") {
            i += 1;
            line = lines[i];
            while line.len() > 0 {
                sts.push(parseMapping(line));
                i += 1;
                line = lines[i];
            }
        } else if line.starts_with("soil-to-fertilizer map:") {
            i += 1;
            line = lines[i];
            while line.len() > 0 {
                stf.push(parseMapping(line));
                i += 1;
                line = lines[i];
            }
        } else if line.starts_with("fertilizer-to-water map:") {
            i += 1;
            line = lines[i];
            while line.len() > 0 {
                ftw.push(parseMapping(line));
                i += 1;
                line = lines[i];
            }
        } else if line.starts_with("water-to-light map:") {
            i += 1;
            line = lines[i];
            while line.len() > 0 {
                wtl.push(parseMapping(line));
                i += 1;
                line = lines[i];
            }
        } else if line.starts_with("light-to-temperature map:") {
            i += 1;
            line = lines[i];
            while line.len() > 0 {
                ltt.push(parseMapping(line));
                i += 1;
                line = lines[i];
            }
        } else if line.starts_with("temperature-to-humidity map:") {
            i += 1;
            line = lines[i];
            while line.len() > 0 {
                tth.push(parseMapping(line));
                i += 1;
                line = lines[i];
            }
        } else if line.starts_with("humidity-to-location map:") {
            i += 1;
            line = lines[i];
            while line.len() > 0 {
                htl.push(parseMapping(line));
                i += 1;
                line = lines[i];
            }
        }
    }

    Maps {seeds, sts, stf, ftw, wtl, ltt, tth, htl}
}

fn parseMapping(line: &str) -> Mapping {

    let nums:Vec<u64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let dst = nums[0];
    let src = nums[1];
    let cnt = nums[2];

    Mapping {dst, src, cnt}
}
