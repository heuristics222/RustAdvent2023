use itertools::Itertools;

#[derive(PartialEq, Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, PartialEq)]
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

    // println!("{:#?}", hsegs);
    // println!("{:#?}", vsegs);

    let mut curx = hsegs.iter().position(|x| *x == 0).unwrap();
    let mut cury = vsegs.iter().position(|x| *x == 0).unwrap();
    let mut curposx = 0;
    let mut curposy = 0;

    let mut grid:Vec<Vec<usize>> = vec![vec![0; hsegs.len()]; vsegs.len()];

    // grid[curx][cury] = 3;
    inputs.iter().enumerate().for_each(|(idx, i)| {

        let amount = match i.dir {
            Direction::Right => {
                curposx = curposx + i.amount;
                hsegs.iter().position(|x| *x == curposx).unwrap() - curx
            },
            Direction::Left => {
                curposx = curposx - i.amount;
                curx - hsegs.iter().position(|x| *x == curposx).unwrap()
            },
            Direction::Up => {
                curposy = curposy - i.amount;
                cury - vsegs.iter().position(|y| *y == curposy).unwrap()
            },
            Direction::Down => {
                curposy = curposy + i.amount;
                vsegs.iter().position(|y| *y == curposy).unwrap() - cury
            },
        };

        (0..amount).for_each(|_| {
            match i.dir {
                Direction::Right => curx += 1,
                Direction::Left => curx -= 1,
                Direction::Up => cury -= 1,
                Direction::Down => cury += 1,
            }
            grid[cury][curx] = if i.dir == Direction::Left || i.dir == Direction::Right {6} else {1};
        });
        grid[cury][curx] = get_elbow(&i.dir, &inputs[(idx + 1) % inputs.len()].dir);
    });

    // println!("{:#?}", grid);

    let perim = inputs.iter().map(|x| x.amount as usize).sum::<usize>() / 2;

    let area = grid.iter().enumerate().map(|(y, row)| {
        if y == grid.len() - 1 {
            return 0;
        }
        let mut inside = false;
        let mut start = 0;
        let height = vsegs[y+1] - vsegs[y];
        let mut sum:isize = 0;
        row.iter().enumerate().for_each(|(idx, val)| {
            match val {
                1 | 2 | 3 => {
                    if !inside {
                        start = hsegs[idx];
                    } else {
                        sum += (hsegs[idx] - start) * height
                    }
                    inside = !inside;
                },
                _ => {},
            }
        });
        sum as usize
    }).sum::<usize>();

    (area + perim + 1).to_string()
}

fn get_elbow(dir1:&Direction, dir2:&Direction) -> usize {
    match (dir1, dir2) {
        (Direction::Right, Direction::Up) => 5,
        (Direction::Right, Direction::Down) => 3,
        (Direction::Left, Direction::Up) => 4,
        (Direction::Left, Direction::Down) => 2,
        (Direction::Up, Direction::Right) => 2,
        (Direction::Up, Direction::Left) => 3,
        (Direction::Down, Direction::Right) => 4,
        (Direction::Down, Direction::Left) => 5,
        _ => panic!(),
    }
}

fn parseInput(input: &str) -> Vec<Instruction> {

    input.lines().map(|line| {
        let split:Vec<&str> = line.split(" ").collect();
        let hex = split[2];

        Instruction {
            amount: isize::from_str_radix(&hex[2..7], 16).unwrap(),
            dir: match &hex[7..8] {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => panic!(),
            }
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
        assert_eq!(result, "952408144115");
    }
}
