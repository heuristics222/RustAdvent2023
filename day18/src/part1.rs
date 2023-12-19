use itertools::Itertools;

#[derive(PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Instruction {
    dir: Direction,
    amount: isize,
}

pub fn execute(input: &str) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    let hsegs:Vec<isize> = inputs.iter().filter(|x| x.dir == Direction::Right || x.dir == Direction::Left)
        .scan(0 as isize, |acc, x| {
            if x.dir == Direction::Right {
                *acc += x.amount;
            } else {
                *acc -= x.amount;
            }
            Some(*acc)
        })
        .sorted()
        .dedup()
        .collect();

    let vsegs:Vec<isize> = inputs.iter().filter(|x| x.dir == Direction::Up || x.dir == Direction::Down)
        .scan(0, |acc, x| {
            if x.dir == Direction::Down {
                *acc += x.amount;
            } else {
                *acc -= x.amount;
            }
            Some(*acc)
        })
        .sorted()
        .dedup()
        .collect();

    let mut grid:Vec<Vec<usize>> = vec![vec![0; (hsegs.last().unwrap() - hsegs.first().unwrap() + 1) as usize]; (vsegs.last().unwrap() - vsegs.first().unwrap() + 1) as usize];

    let mut curx = (-hsegs.first().unwrap()) as usize;
    let mut cury = (-vsegs.first().unwrap()) as usize;
    inputs.iter().enumerate().for_each(|(idx, i)| {
        (0..i.amount).for_each(|_| {
            match i.dir {
                Direction::Right => curx += 1,
                Direction::Left => curx -= 1,
                Direction::Up => cury -= 1,
                Direction::Down => cury += 1,
            }
            grid[cury][curx] = if i.dir == Direction::Left || i.dir == Direction::Right {6} else {1};
        });
        match (&i.dir, &inputs[(idx + 1) % inputs.len()].dir) {
            (Direction::Right, Direction::Up) => grid[cury][curx] = 5,
            (Direction::Right, Direction::Down) => grid[cury][curx] = 3,
            (Direction::Left, Direction::Up) => grid[cury][curx] = 4,
            (Direction::Left, Direction::Down) => grid[cury][curx] = 2,
            (Direction::Up, Direction::Right) => grid[cury][curx] = 2,
            (Direction::Up, Direction::Left) => grid[cury][curx] = 3,
            (Direction::Down, Direction::Right) => grid[cury][curx] = 4,
            (Direction::Down, Direction::Left) => grid[cury][curx] = 5,
            _ => panic!(),
        }
    });

    grid.iter().map(|row| {
        let mut inside = false;
        row.iter().filter(|val| {
            match val {
                0 => inside,
                1 => {
                    inside = !inside;
                    true
                },
                2 => {
                    inside = !inside;
                    true
                }
                3 => {
                    inside = !inside;
                    true
                }
                4 => true,
                5 => true,
                6 => true,
                _ => panic!(),
            }
        }).count()
    }).sum::<usize>().to_string()
}

fn parseInput(input: &str) -> Vec<Instruction> {

    input.lines().map(|line| {
        let split:Vec<&str> = line.split(" ").collect();
        Instruction {
            dir: match split[0] {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!(),
            },
            amount: split[1].parse::<isize>().unwrap()
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute("
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
");
        assert_eq!(result, "62");
    }
}
