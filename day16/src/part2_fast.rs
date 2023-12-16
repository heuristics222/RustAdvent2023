use itertools::Itertools;
use num::integer::Roots;
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
    let len = inputs.len().sqrt();
    // println!("{:#?}", inputs);
    // println!("{}", len);

    static DIRECTIONS: [FromDirection; 4] = [FromDirection::Left, FromDirection::Right, FromDirection::Top, FromDirection::Bottom];
    (0..len).cartesian_product(DIRECTIONS.iter()).par_bridge().map(|(idx, dir)| {
        let mut energized = vec![false; len*len];
        let mut visited = vec![false; len*len];

        match dir {
            FromDirection::Left => traverse(&inputs, 0, idx, *dir, &mut energized, &mut visited, len),
            FromDirection::Right => traverse(&inputs, len - 1, idx, *dir, &mut energized, &mut visited, len),
            FromDirection::Top => traverse(&inputs, idx, 0, *dir, &mut energized, &mut visited, len),
            FromDirection::Bottom => traverse(&inputs, idx, len - 1, *dir, &mut energized, &mut visited, len),
        }

        energized.iter().filter(|x| **x).count()
    }).reduce(|| 0, |a,b| a.max(b)).to_string()
}

fn traverse(grid: &[u8], x:usize, y:usize, dir: FromDirection, energized: &mut Vec<bool>, visited: &mut Vec<bool>, len: usize) {
    let idx = y * len + x;
    energized[idx] = true;

    match dir {
        FromDirection::Left => {
            match grid[idx] {
                b'\\' => {
                    if y+1 < len {
                        traverse(grid, x, y+1, FromDirection::Top, energized, visited, len);
                    }
                },
                b'/' => {
                    if y > 0 {
                        traverse(grid, x, y-1, FromDirection::Bottom, energized, visited, len);
                    }
                },
                b'|' => {
                    if visited[idx] {
                        return;
                    }
                    visited[idx] = true;

                    if y+1 < len {
                        traverse(grid, x, y+1, FromDirection::Top, energized, visited, len);
                    }
                    if y > 0 {
                        traverse(grid, x, y-1, FromDirection::Bottom, energized, visited, len);
                    }
                },
                b'-' => {
                    if x+1 < len {
                        traverse(grid, x+1, y, FromDirection::Left, energized, visited, len);
                    }
                },
                b'.' => {
                    if x+1 < len {
                        traverse(grid, x+1, y, FromDirection::Left, energized, visited, len);
                    }
                },
                _ => panic!(),
            }
        },
        FromDirection::Right => {
            match grid[idx] {
                b'\\' => {
                    if y > 0 {
                        traverse(grid, x, y-1, FromDirection::Bottom, energized, visited, len);
                    }
                },
                b'/' => {
                    if y+1 < len {
                        traverse(grid, x, y+1, FromDirection::Top, energized, visited, len);
                    }
                },
                b'|' => {
                    if visited[idx] {
                        return;
                    }
                    visited[idx] = true;
                    if y+1 < len {
                        traverse(grid, x, y+1, FromDirection::Top, energized, visited, len);
                    }
                    if y > 0 {
                        traverse(grid, x, y-1, FromDirection::Bottom, energized, visited, len);
                    }
                },
                b'-' => {
                    if x > 0 {
                        traverse(grid, x-1, y, FromDirection::Right, energized, visited, len);
                    }
                },
                b'.' => {
                    if x > 0 {
                        traverse(grid, x-1, y, FromDirection::Right, energized, visited, len);
                    }
                },
                _ => panic!(),
            }
        },
        FromDirection::Top => {
            match grid[idx] {
                b'\\' => {
                    if x+1 < len {
                        traverse(grid, x+1, y, FromDirection::Left, energized, visited, len);
                    }
                },
                b'/' => {
                    if x > 0 {
                        traverse(grid, x-1, y, FromDirection::Right, energized, visited, len);
                    }
                },
                b'|' => {
                    if y+1 < len {
                        traverse(grid, x, y+1, FromDirection::Top, energized, visited, len);
                    }
                },
                b'-' => {
                    if visited[idx] {
                        return;
                    }
                    visited[idx] = true;
                    if x > 0 {
                        traverse(grid, x-1, y, FromDirection::Right, energized, visited, len);
                    }
                    if x+1 < len {
                        traverse(grid, x+1, y, FromDirection::Left, energized, visited, len);
                    }
                },
                b'.' => {
                    if y+1 < len {
                        traverse(grid, x, y+1, FromDirection::Top, energized, visited, len);
                    }
                },
                _ => panic!(),
            }
        },
        FromDirection::Bottom => {
            match grid[idx] {
                b'\\' => {
                    if x > 0 {
                        traverse(grid, x-1, y, FromDirection::Right, energized, visited, len);
                    }
                },
                b'/' => {
                    if x+1 < len {
                        traverse(grid, x+1, y, FromDirection::Left, energized, visited, len);
                    }
                },
                b'|' => {
                    if y > 0 {
                        traverse(grid, x, y-1, FromDirection::Bottom, energized, visited, len);
                    }
                },
                b'-' => {
                    if visited[idx] {
                        return;
                    }
                    visited[idx] = true;
                    if x > 0 {
                        traverse(grid, x-1, y, FromDirection::Right, energized, visited, len);
                    }
                    if x+1 < len {
                        traverse(grid, x+1, y, FromDirection::Left, energized, visited, len);
                    }
                },
                b'.' => {
                    if y > 0 {
                        traverse(grid, x, y-1, FromDirection::Bottom, energized, visited, len);
                    }
                },
                _ => panic!(),
            }
        },
    }

}

fn parseInput(input: &str) -> Vec<u8> {
    input.lines().flat_map(|x| x.bytes()).collect()
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
