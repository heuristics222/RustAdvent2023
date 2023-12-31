
#[derive(Debug)]
struct BestTime {
    time: u32,
    distance: u32,
}

pub fn main() -> String {
    let input = include_str!("./input.txt");
    let output = part1(input);
    output.to_string()
}

fn part1(input: &str) -> u64 {
    let bestTimes = parseInput(input);
    let mut total = 1;

    for bestTime in bestTimes {
        let mut winningTimes = 0;
        for chargeTime in 1..bestTime.time {
            if chargeTime * (bestTime.time - chargeTime) > bestTime.distance {
                winningTimes += 1;
            }
        }
        total *= winningTimes;
    }

    total
}

fn parseInput(input: &str) -> Vec<BestTime> {
    let lines:Vec<&str> = input.lines().collect();
    let times:Vec<u32> = lines[0].split_once(":").unwrap().1.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let distances:Vec<u32> = lines[1].split_once(":").unwrap().1.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut bestTimes:Vec<BestTime> = Default::default();

    for i in 0..times.len() {
        bestTimes.push(BestTime {time: times[i], distance: distances[i]});
    }

    bestTimes
}
