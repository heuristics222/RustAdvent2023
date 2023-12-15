
#[derive(Debug)]
struct BestTime {
    time: u64,
    distance: u64,
}

pub fn main() -> String {
    let input = include_str!("./input.txt");
    let output = part1(input);
    output.to_string()
}

fn part1(input: &str) -> u64 {
    let bestTime = parseInput(input);

    let mut winningTimes = 0;
    for chargeTime in 1..bestTime.time {
        if chargeTime * (bestTime.time - chargeTime) > bestTime.distance {
            winningTimes += 1;
        }
    }

    winningTimes
}

fn parseInput(input: &str) -> BestTime {
    let lines:Vec<&str> = input.lines().collect();
    let time:u64 = lines[0].split_once(":").unwrap().1.split_whitespace().fold(String::new(), |a, b| a + b).parse().unwrap();
    let distance:u64 = lines[1].split_once(":").unwrap().1.split_whitespace().fold(String::new(), |a, b| a + b).parse().unwrap();


    BestTime {time, distance}
}
