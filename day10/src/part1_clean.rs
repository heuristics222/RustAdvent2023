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

    (map.len() / 2).to_string()
}

fn parseInput(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|x| x.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute(
"
7-F7-
.FJ|7
FJLL7
|F--J
LJ.LJ
",
        (0, 2));
        assert_eq!(result, "8");
    }
}
