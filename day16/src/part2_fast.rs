use itertools::Itertools;
use rayon::prelude::*;

#[derive(Copy, Clone)]
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
    (0..inputs[0].len()).cartesian_product(DIRECTIONS.iter()).par_bridge().map(|(idx, dir)| {
        let mut energized = vec![vec![false; inputs[0].len()]; inputs.len()];
        let mut visited: Vec<Vec<bool>> = vec![vec![false; inputs[0].len()]; inputs.len()];

        match dir {
            FromDirection::Left => traverse(&inputs, 0, idx, *dir, &mut energized, &mut visited),
            FromDirection::Right => traverse(&inputs, inputs[0].len() - 1, idx, *dir, &mut energized, &mut visited),
            FromDirection::Top => traverse(&inputs, idx, 0, *dir, &mut energized, &mut visited),
            FromDirection::Bottom => traverse(&inputs, idx, inputs.len() - 1, *dir, &mut energized, &mut visited),
        }

        energized.iter().flatten().filter(|x| **x).count()
    }).reduce(|| 0, |a,b| a.max(b)).to_string()
}

fn traverse(grid: &Vec<Vec<char>>, x_s:usize, y_s:usize, dir_s: FromDirection, energized: &mut Vec<Vec<bool>>, visited: &mut Vec<Vec<bool>>) {

    let mut q: Vec<(usize, usize, FromDirection)> = Default::default();

    q.push((x_s, y_s, dir_s));

    while !q.is_empty() {
        let (x,y,dir) = q.pop().unwrap();
        energized[y][x] = true;

        match dir {
            FromDirection::Left => {
                match grid[y][x] {
                    '\\' => {
                        if y+1 < grid.len() {
                            q.push((x, y+1, FromDirection::Top));
                        }
                    },
                    '/' => {
                        if y > 0 {
                            q.push((x, y-1, FromDirection::Bottom));
                        }
                    },
                    '|' => {
                        if visited[y][x] {
                            continue;
                        }
                        visited[y][x] = true;

                        if y+1 < grid.len() {
                            q.push((x, y+1, FromDirection::Top));
                        }
                        if y > 0 {
                            q.push((x, y-1, FromDirection::Bottom));
                        }
                    },
                    '-' => {
                        if x+1 < grid[0].len() {
                            q.push((x+1, y, FromDirection::Left));
                        }
                    },
                    '.' => {
                        if x+1 < grid[0].len() {
                            q.push((x+1, y, FromDirection::Left));
                        }
                    },
                    _ => panic!(),
                }
            },
            FromDirection::Right => {
                match grid[y][x] {
                    '\\' => {
                        if y > 0 {
                            q.push((x, y-1, FromDirection::Bottom));
                        }
                    },
                    '/' => {
                        if y+1 < grid.len() {
                            q.push((x, y+1, FromDirection::Top));
                        }
                    },
                    '|' => {
                        if visited[y][x] {
                            continue;
                        }
                        visited[y][x] = true;
                        if y+1 < grid.len() {
                            q.push((x, y+1, FromDirection::Top));
                        }
                        if y > 0 {
                            q.push((x, y-1, FromDirection::Bottom));
                        }
                    },
                    '-' => {
                        if x > 0 {
                            q.push((x-1, y, FromDirection::Right));
                        }
                    },
                    '.' => {
                        if x > 0 {
                            q.push((x-1, y, FromDirection::Right));
                        }
                    },
                    _ => panic!(),
                }
            },
            FromDirection::Top => {
                match grid[y][x] {
                    '\\' => {
                        if x+1 < grid[0].len() {
                            q.push((x+1, y, FromDirection::Left));
                        }
                    },
                    '/' => {
                        if x > 0 {
                            q.push((x-1, y, FromDirection::Right));
                        }
                    },
                    '|' => {
                        if y+1 < grid.len() {
                            q.push((x, y+1, FromDirection::Top));
                        }
                    },
                    '-' => {
                        if visited[y][x] {
                            continue;
                        }
                        visited[y][x] = true;
                        if x > 0 {
                            q.push((x-1, y, FromDirection::Right));
                        }
                        if x+1 < grid[0].len() {
                            q.push((x+1, y, FromDirection::Left));
                        }
                    },
                    '.' => {
                        if y+1 < grid.len() {
                            q.push((x, y+1, FromDirection::Top));
                        }
                    },
                    _ => panic!(),
                }
            },
            FromDirection::Bottom => {
                match grid[y][x] {
                    '\\' => {
                        if x > 0 {
                            q.push((x-1, y, FromDirection::Right));
                        }
                    },
                    '/' => {
                        if x+1 < grid[0].len() {
                            q.push((x+1, y, FromDirection::Left));
                        }
                    },
                    '|' => {
                        if y > 0 {
                            q.push((x, y-1, FromDirection::Bottom));
                        }
                    },
                    '-' => {
                        if visited[y][x] {
                            continue;
                        }
                        visited[y][x] = true;
                        if x > 0 {
                            q.push((x-1, y, FromDirection::Right));
                        }
                        if x+1 < grid[0].len() {
                            q.push((x+1, y, FromDirection::Left));
                        }
                    },
                    '.' => {
                        if y > 0 {
                            q.push((x, y-1, FromDirection::Bottom));
                        }
                    },
                    _ => panic!(),
                }
            },
        }
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
