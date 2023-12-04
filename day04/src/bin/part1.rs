struct Card {
    mine: Vec<i32>,
    winning: Vec<i32>,
}

impl Card {
    fn score(&self) -> i32 {
        let mut count = 0;
        for x in &self.mine {
            if self.winning.contains(&x) {
                count += 1;
            }
        }

        match count {
            0 => 0,
            _ => 2_i32.pow(count-1)
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let cards = parseInput(input);

    cards.iter().map(|x| x.score()).sum()
}

fn parseInput(input: &str) -> Vec<Card> {
    let mut lines:Vec<&str> = input.lines().collect();

    lines = lines.iter().map(|x| x.split_once(":").unwrap().1).collect();


    lines.iter().map(|x| parseCard(x)).collect()
    //let mine = lines.iter().map(|x| x.split_once("|").unwrap().0.split)
}

fn parseCard(line: &str) -> Card {
    let split = line.split_once("|").unwrap();
    let mine:Vec<i32> = split.0.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let winning:Vec<i32> = split.1.split_whitespace().map(|x| x.parse().unwrap()).collect();

    Card {mine, winning}
}
