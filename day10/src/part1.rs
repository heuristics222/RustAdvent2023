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

    // println!("{startX} {startY}");

    let mut count = 1;
    let mut x = startX;
    let mut y = startY+1;
    let mut prevX = startX;
    let mut prevY = startY;

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

        prevX = x;
        prevY = y;
        x = newX;
        y = newY;
        // println!("{x}, {y}");
        count += 1;
    }

    (count / 2).to_string()
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
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
");
        assert_eq!(result, "8");
    }
}
