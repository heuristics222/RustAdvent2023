use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
#[derive(Eq)]
struct Input {
    hand: Vec<char>,
    bet: u64,
}

#[derive(Debug)]
#[derive(Ord, PartialOrd, PartialEq, Eq)]
enum HandType {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.hand.eq(&other.hand)
    }
}

impl Ord for Input {
    fn cmp(&self, other: &Self) -> Ordering {
        let myHandType = getHandType(&self.hand);
        let otherHandType = getHandType(&other.hand);

        if myHandType == otherHandType {
            for i in 0..self.hand.len() {
                if self.hand[i] != other.hand[i] {
                    return getCharValue(self.hand[i]).cmp(&getCharValue(other.hand[i]));
                }
            }
            Ordering::Equal
        } else {
            myHandType.cmp(&otherHandType)
        }
    }
}

impl PartialOrd for Input {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u64 {
    let mut inputs = parseInput(input);
    // println!("{:#?}", inputs);

    inputs.sort();
    let hands:Vec<(&Input, HandType)> = inputs.iter().map(|x| (x, getHandType(&x.hand))).collect();

    hands.iter().enumerate().map(|x| (x.0 + 1) as u64 * x.1.0.bet).sum()
}

fn parseInput(input: &str) -> Vec<Input> {
    input.lines().into_iter().map(|x| parseLine(x)).collect()
}

fn parseLine(line: &str) -> Input {
    let split = line.split_once(" ");
    Input { hand: split.unwrap().0.chars().collect(), bet: split.unwrap().1.parse().unwrap() }
}

fn getHandType(hand: &Vec<char>) -> HandType {
    let mut map: HashMap<char, u64> = Default::default();

    for x in hand {
        map.insert(*x, map.get(&x).unwrap_or(&0) + 1);
    }

    if map.len() == 5 {
        HandType::High
    } else if map.len() == 4 {
        HandType::One
    } else if map.len() == 3 {
        if map.values().any(|&x| x == 3) {
            HandType::Three
        } else {
            HandType::Two
        }
    } else if map.len() == 2 {
        if map.values().any(|&x| x == 3) {
            HandType::Full
        } else {
            HandType::Four
        }
    } else {
        HandType::Five
    }

}

fn getCharValue(ch: char) -> u64 {
    match ch {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!(),
    }
}
