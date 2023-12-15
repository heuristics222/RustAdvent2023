pub fn execute(input: &str) -> String {
    let mut inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    tilt_north(&mut inputs);

    inputs.iter().enumerate()
        .map(|(i, line)| line.iter().filter(|ch| **ch == 'O').map(|_| inputs.len() - i).sum::<usize>()).sum::<usize>().to_string()

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
        assert_eq!(result, "136");
    }
}
