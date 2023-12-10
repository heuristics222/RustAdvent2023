use std::collections::{HashSet, VecDeque};

pub fn execute(input: &str, start: (u32, u32)) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    let mut deq:VecDeque<(u32, u32)> = Default::default();
    let mut map:HashSet<(u32, u32)> = Default::default();
    deq.push_back(start);
    while let Some(x) = deq.pop_front() {
        if !map.contains(&x) {
            map.insert(x.clone());
            match inputs[x.1 as usize][x.0 as usize] {
                '-' => {deq.push_back((x.0+1, x.1)); deq.push_back((x.0-1, x.1))},
                '|' => {deq.push_back((x.0, x.1+1)); deq.push_back((x.0, x.1-1))},
                'F' => {deq.push_back((x.0+1, x.1)); deq.push_back((x.0, x.1+1))},
                'L' => {deq.push_back((x.0+1, x.1)); deq.push_back((x.0, x.1-1))},
                '7' => {deq.push_back((x.0-1, x.1)); deq.push_back((x.0, x.1+1))},
                'J' => {deq.push_back((x.0-1, x.1)); deq.push_back((x.0, x.1-1))},
                _ => panic!(),
            }
        }
    }

    countInner(inputs, map).to_string()
}

fn countInner(input: Vec<Vec<char>>, map: HashSet<(u32, u32)>) -> u32 {

    let mut count = 0;

    for (y, l) in input.iter().enumerate() {
        let mut lastChar = '.';
        let mut inside = false;
        for (x, ch) in l.iter().enumerate() {
            if map.contains(&(x as u32, y as u32)) {
                match *ch {
                    '|' => inside = !inside,
                    '-' => {},
                    'F' => lastChar = 'F',
                    'L' => lastChar = 'L',
                    'J' => if lastChar == 'F' { inside = !inside },
                    '7' => if lastChar == 'L' { inside = !inside },
                    _ => panic!(),
                }
            } else {
                if inside {
                    count += 1;
                }
            }
        }
    }
    count
}

fn parseInput(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|x| x.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute("
FF7F7F7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
        ", (4, 0));
        assert_eq!(result, "10");
    }
}
