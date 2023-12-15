use std::collections::HashMap;

pub fn execute(input: &str) -> String {
    let mut inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    let mut cache:HashMap<Vec<Vec<char>>, Vec<Vec<char>>> = HashMap::new();
    let mut i = 0;
    loop {

        if cache.get(&inputs).is_some() {
            break;
        }

        let initial = inputs.clone();
        tilt_north(&mut inputs);
        tilt_west(&mut inputs);
        tilt_south(&mut inputs);
        tilt_east(&mut inputs);

        cache.insert(initial, inputs.clone());
        i += 1;
    }

    let mut cycled = &inputs;
    let cycle_start = i;
    let cycle_start_grid = inputs.clone();
    while cycled != &cycle_start_grid || cycle_start == i {
        cycled = cache.get(cycled).unwrap();

        i += 1;
    }

    let cycle_len = i - cycle_start;
    while i + cycle_len < 1000000000 {
        i += cycle_len;
    }

    while i < 1000000000 {
        cycled = cache.get(cycled).unwrap();

        i += 1;
    }


    value(cycled).to_string()
}

fn value(input: &Vec<Vec<char>>) -> usize {
    input.iter().enumerate()
        .map(|(i, line)| line.iter().filter(|ch| **ch == 'O').map(|_| input.len() - i).sum::<usize>()).sum::<usize>()
}

fn tilt_north(input: &mut Vec<Vec<char>>) {
    for x in 0..input[0].len() {
        for y in 0..input.len() {
            if input[y][x] == 'O' {
                if y > 0 {
                    for z in (0..y).rev() {
                        if input[z][x] == '.' {
                            input[z][x] = 'O';
                            input[z+1][x] = '.';
                        } else if input[z][x] == '#' {
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn tilt_west(input: &mut Vec<Vec<char>>) {
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 'O' {
                if x > 0 {
                    for z in (0..x).rev() {
                        if input[y][z] == '.' {
                            input[y][z] = 'O';
                            input[y][z+1] = '.';
                        } else if input[y][z] == '#' {
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn tilt_south(input: &mut Vec<Vec<char>>) {
    for x in 0..input[0].len() {
        for y in (0..input.len()).rev() {
            if input[y][x] == 'O' {
                if y < input.len() - 1 {
                    for z in y+1..input.len() {
                        if input[z][x] == '.' {
                            input[z][x] = 'O';
                            input[z-1][x] = '.';
                        } else if input[z][x] == '#' {
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn tilt_east(input: &mut Vec<Vec<char>>) {
    for y in 0..input.len() {
        for x in (0..input[0].len()).rev() {
            if input[y][x] == 'O' {
                if x < input[0].len() - 1 {
                    for z in x+1..input[0].len() {
                        if input[y][z] == '.' {
                            input[y][z] = 'O';
                            input[y][z-1] = '.';
                        } else if input[y][z] == '#' {
                            break;
                        }
                    }
                }
            }
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
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
");
        assert_eq!(result, "64");
    }
}
