use std::collections::BinaryHeap;


#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn val(&self) -> usize {
        match self {
            Direction::Left => 0,
            Direction::Right => 1,
            Direction::Up => 2,
            Direction::Down => 3,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    x: usize,
    y: usize,
    consec: usize,
    dir: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn execute(input: &str) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    traverse(&inputs).to_string()
}

fn push_into(dist: &mut Vec<Vec<Vec<Vec<usize>>>>, heap: &mut BinaryHeap<State>, state: &State) {
    if state.cost < dist[state.y][state.x][state.dir.val()][state.consec] {
        heap.push(*state);
        dist[state.y][state.x][state.dir.val()][state.consec] = state.cost;
    }
}

fn traverse(grid: &Vec<Vec<usize>>) -> usize {

    let mut heap = BinaryHeap::new();

    let mut dist = vec![vec![vec![vec![usize::MAX; 11]; 4]; grid[0].len()]; grid.len()];

    push_into(&mut dist, &mut heap, &State {x:0, y:0, cost:0, consec:0, dir:Direction::Down});
    push_into(&mut dist, &mut heap, &State {x:0, y:0, cost:0, consec:0, dir:Direction::Right});


    while let Some(State { cost, x, y, consec, dir}) = heap.pop() {
        if x == grid[0].len() - 1 && y == grid.len() - 1 && consec >= 4 {
            return cost;
        }

        if cost > dist[y][x][dir.val()][consec] {
            continue;
        }

        match dir {
            Direction::Left => {
                if y+1 < grid.len() && consec >= 4 {
                    push_into(&mut dist, &mut heap, &State {x:x, y:y+1, cost:cost + grid[y+1][x], consec:1, dir:Direction::Down});
                }
                if y >= 1 && consec >= 4 {
                    push_into(&mut dist, &mut heap, &State {x:x, y:y-1, cost:cost + grid[y-1][x], consec:1, dir:Direction::Up});
                }
                if consec < 10 && x >= 1 {
                    push_into(&mut dist, &mut heap, &State {x:x-1, y:y, cost:cost + grid[y][x-1], consec:consec+1, dir:Direction::Left});
                }
            },
            Direction::Right => {
                if y+1 < grid.len() && consec >= 4 {
                    push_into(&mut dist, &mut heap, &State {x:x, y:y+1, cost:cost + grid[y+1][x], consec:1, dir:Direction::Down});
                }
                if y >= 1 && consec >= 4 {
                    push_into(&mut dist, &mut heap, &State {x:x, y:y-1, cost:cost + grid[y-1][x], consec:1, dir:Direction::Up});
                }
                if consec < 10 && x+1 < grid[0].len() {
                    push_into(&mut dist, &mut heap, &State {x:x+1, y:y, cost:cost + grid[y][x+1], consec:consec+1, dir:Direction::Right});
                }
            },
            Direction::Up => {
                if x+1 < grid[0].len() && consec >= 4 {
                    push_into(&mut dist, &mut heap, &State {x:x+1, y:y, cost:cost + grid[y][x+1], consec:1, dir:Direction::Right});
                }
                if x >= 1 && consec >= 4 {
                    push_into(&mut dist, &mut heap, &State {x:x-1, y:y, cost:cost + grid[y][x-1], consec:1, dir:Direction::Left});
                }
                if consec < 10 && y >= 1 {
                    push_into(&mut dist, &mut heap, &State {x:x, y:y-1, cost:cost + grid[y-1][x], consec:consec+1, dir:Direction::Up});
                }
            },
            Direction::Down => {
                if x+1 < grid[0].len() && consec >= 4 {
                    push_into(&mut dist, &mut heap, &State {x:x+1, y:y, cost:cost + grid[y][x+1], consec:1, dir:Direction::Right});
                }
                if x >= 1 && consec >= 4 {
                    push_into(&mut dist, &mut heap, &State {x:x-1, y:y, cost:cost + grid[y][x-1], consec:1, dir:Direction::Left});
                }
                if consec < 10 && y+1 < grid.len() {
                    push_into(&mut dist, &mut heap, &State {x:x, y:y+1, cost:cost + grid[y+1][x], consec:consec+1, dir:Direction::Down});
                }
            },
        }
    }

    0
}

fn parseInput(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as usize).collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute("
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
");
        assert_eq!(result, "94");
    }

    #[test]
    fn it_works2() {
        let result = execute("
111111111111
999999999991
999999999991
999999999991
999999999991
");
        assert_eq!(result, "71");
    }

    #[test]
    fn it_works3() {
        let result = execute("
1111111111111
9991999919991
9991999919991
9991999919991
9991111119991
");
        assert_eq!(result, "34");
    }
}
