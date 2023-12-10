pub fn execute(input: &str) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    let mut startX = 0;
    let mut startY = 0;

    for (i, l) in inputs.iter().enumerate() {
        for (j, ch) in l.iter().enumerate() {
            if *ch == 'S' {
                startY = i;
                startX = j;
            }
        }
    }

    let mut cleaned = vec![vec!['.'; inputs[0].len()]; inputs.len()];

    // println!("{startX} {startY}");

    let mut count = 1;
    let mut x = startX;
    let mut y = startY+1;
    let mut prevX = startX;
    let mut prevY = startY;

    cleaned[startY][startX] = '|';

    while x != startX || y != startY || count > 0 {
        let ch = inputs[y][x];
        let mut newX = x;
        let mut newY = y;
        match ch {
            '-' => {
                if x < prevX {
                    newX = x - 1;
                } else {
                    newX = x + 1;
                }
            },
            '|' => {
                if y < prevY {
                    newY = y - 1;
                } else {
                    newY = y + 1;
                }
            },
            'J' => {
                if x > prevX {
                    newY = y - 1;
                } else {
                    newX = x - 1;
                }
            },
            'L' => {
                if x < prevX {
                    newY = y - 1;
                } else {
                    newX = x + 1;
                }
            },
            'F' => {
                if x < prevX {
                    newY = y + 1;
                } else {
                    newX = x + 1;
                }
            },
            '7' => {
                if x > prevX {
                    newY = y + 1;
                } else {
                    newX = x - 1;
                }
            },
            'S' => {
                break;
            }
            _ => panic!(),
        }
        cleaned[y][x] = inputs[y][x];

        prevX = x;
        prevY = y;
        x = newX;
        y = newY;
        // println!("{x}, {y}");
        count += 1;
    }

    // cleaned.iter().for_each(|x| println!("{}", x.iter().collect::<String>()));

    countInner(cleaned).to_string()
}

fn countInner(input: Vec<Vec<char>>) -> u32 {

    let mut count = 0;

    for l in input.iter() {
        let mut lastChar = '.';
        let mut inside = false;
        for ch in l.iter() {
            match *ch {
                '|' => inside = !inside,
                '-' => {},
                '.' => {
                    if inside {
                        count += 1;
                    }
                },
                'F' => lastChar = 'F',
                'L' => lastChar = 'L',
                'J' => if lastChar == 'F' { inside = !inside },
                '7' => if lastChar == 'L' { inside = !inside },
                _ => panic!(),
            }
        }
    }
    count
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
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
");
        assert_eq!(result, "10");
    }
}
