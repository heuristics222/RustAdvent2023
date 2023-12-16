use std::collections::HashSet;
use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum FromDirection {
    Left,
    Right,
    Top,
    Bottom,
}

pub fn execute(input: &str) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    static DIRECTIONS: [FromDirection; 4] = [FromDirection::Left, FromDirection::Right, FromDirection::Top, FromDirection::Bottom];
    (0..inputs[0].len()).cartesian_product(DIRECTIONS.iter()).map(|(idx, dir)| {
        let mut energized = vec![vec![false; inputs[0].len()]; inputs.len()];
        let mut visited: HashSet<(i32, i32, FromDirection)> = Default::default();

        match dir {
            FromDirection::Left => traverse(&inputs, 0, idx as i32, *dir, &mut energized, &mut visited),
            FromDirection::Right => traverse(&inputs, (inputs[0].len() - 1) as i32, idx as i32, *dir, &mut energized, &mut visited),
            FromDirection::Top => traverse(&inputs, idx as i32, 0, *dir, &mut energized, &mut visited),
            FromDirection::Bottom => traverse(&inputs, idx as i32, (inputs.len() - 1) as i32, *dir, &mut energized, &mut visited),
        }

        energized.iter().flatten().filter(|x| **x).count()
    }).fold(0, |a,b| a.max(b)).to_string()
}

fn traverse(grid: &Vec<Vec<char>>, x:i32, y:i32, dir: FromDirection, energized: &mut Vec<Vec<bool>>, visited: &mut HashSet<(i32, i32, FromDirection)>) {

    if x < 0 || y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 {
        return;
    }
    let key = (x, y, dir);
    if visited.contains(&key) {
        return;
    }
    visited.insert(key);

    energized[y as usize][x as usize] = true;

    match dir {
        FromDirection::Left => {
            match grid[y as usize][x as usize] {
                '\\' => {
                    traverse(grid, x, y+1, FromDirection::Top, energized, visited);
                },
                '/' => {
                    traverse(grid, x, y-1, FromDirection::Bottom, energized, visited);
                },
                '|' => {
                    traverse(grid, x, y+1, FromDirection::Top, energized, visited);
                    traverse(grid, x, y-1, FromDirection::Bottom, energized, visited);
                },
                '-' => {
                    traverse(grid, x+1, y, FromDirection::Left, energized, visited);
                },
                '.' => {
                    traverse(grid, x+1, y, FromDirection::Left, energized, visited);
                },
                _ => panic!(),
            }
        },
        FromDirection::Right => {
            match grid[y as usize][x as usize] {
                '\\' => {
                    traverse(grid, x, y-1, FromDirection::Bottom, energized, visited);
                },
                '/' => {
                    traverse(grid, x, y+1, FromDirection::Top, energized, visited);
                },
                '|' => {
                    traverse(grid, x, y+1, FromDirection::Top, energized, visited);
                    traverse(grid, x, y-1, FromDirection::Bottom, energized, visited);
                },
                '-' => {
                    traverse(grid, x-1, y, FromDirection::Right, energized, visited);
                },
                '.' => {
                    traverse(grid, x-1, y, FromDirection::Right, energized, visited);
                },
                _ => panic!(),
            }
        },
        FromDirection::Top => {
            match grid[y as usize][x as usize] {
                '\\' => {
                    traverse(grid, x+1, y, FromDirection::Left, energized, visited);
                },
                '/' => {
                    traverse(grid, x-1, y, FromDirection::Right, energized, visited);
                },
                '|' => {
                    traverse(grid, x, y+1, FromDirection::Top, energized, visited);
                },
                '-' => {
                    traverse(grid, x-1, y, FromDirection::Right, energized, visited);
                    traverse(grid, x+1, y, FromDirection::Left, energized, visited);
                },
                '.' => {
                    traverse(grid, x, y+1, FromDirection::Top, energized, visited);
                },
                _ => panic!(),
            }
        },
        FromDirection::Bottom => {
            match grid[y as usize][x as usize] {
                '\\' => {
                    traverse(grid, x-1, y, FromDirection::Right, energized, visited);
                },
                '/' => {
                    traverse(grid, x+1, y, FromDirection::Left, energized, visited);
                },
                '|' => {
                    traverse(grid, x, y-1, FromDirection::Bottom, energized, visited);
                },
                '-' => {
                    traverse(grid, x-1, y, FromDirection::Right, energized, visited);
                    traverse(grid, x+1, y, FromDirection::Left, energized, visited);
                },
                '.' => {
                    traverse(grid, x, y-1, FromDirection::Bottom, energized, visited);
                },
                _ => panic!(),
            }
        },
    }

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
.|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
");
        assert_eq!(result, "51");
    }
}
