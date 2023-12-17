use std::collections::BinaryHeap;

pub fn execute(input: &str) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    traverse(&inputs).to_string()
}

fn push_into(grid: &Vec<Vec<usize>>, dist: &mut Vec<usize>, heap: &mut BinaryHeap<isize>, x:usize, y:usize, cost:usize, dir:usize) {
    let state: usize = cost << 33 | dir << 32 | x << 16 | y;
    let idx = y * grid[0].len() + x + dir * grid[0].len() * grid.len();
    if cost < dist[idx] {
        heap.push(-(state as isize));
        dist[idx] = cost;
    }
}

fn traverse(grid: &Vec<Vec<usize>>) -> usize {
    let mut heap = BinaryHeap::new();
    let mut dist = vec![usize::MAX; 2 * grid[0].len() * grid.len()];

    push_into(grid, &mut dist, &mut heap, 0, 0, 1, 0);
    push_into(grid, &mut dist, &mut heap, 0, 0, 1, 1);

    while let Some(state) = heap.pop() {
        let state:usize = (-state) as usize;
        let cost = (state & 0xFFFFFFFE00000000) >> 33;
        let dir = (state & 0x0000000100000000) >> 32;
        let x = (state & 0x00000000FFFF0000) >> 16;
        let y = state & 0x000000000000FFFF;
        if x == grid[0].len() - 1 && y == grid.len() - 1 {
            return cost - 1;
        }

        if cost > dist[y * grid[0].len() + x + dir * grid[0].len() * grid.len()] {
            continue;
        }

        match dir {
            1 => {
                let mut new_cost = cost;
                (1..=10).for_each(|i| {
                    if y+i < grid.len() {
                        new_cost = new_cost + grid[y+i][x];
                        if i >= 4 {
                            push_into(grid, &mut dist, &mut heap, x, y+i, new_cost, 0);
                        }
                    }

                });
                let mut new_cost = cost;
                (1..=10).for_each(|i| {
                    if y >= i {
                        new_cost = new_cost + grid[y-i][x];
                        if i >= 4 {
                            push_into(grid, &mut dist, &mut heap, x, y-i, new_cost, 0);
                        }
                    }
                });
            },
            0 => {
                let mut new_cost = cost;
                (1..=10).for_each(|i| {
                    if x+i < grid[0].len() {
                        new_cost = new_cost + grid[y][x+i];
                        if i >= 4 {
                            push_into(grid, &mut dist, &mut heap, x+i, y, new_cost, 1);
                        }
                    }

                });
                let mut new_cost = cost;
                (1..=10).for_each(|i| {
                    if x >= i {
                        new_cost = new_cost + grid[y][x-i];
                        if i >= 4 {
                            push_into(grid, &mut dist, &mut heap, x-i, y, new_cost, 1);
                        }
                    }
                });
            },
            _ => panic!(),
        }
    }

    panic!();
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

    #[test]
    fn it_works4() {
        let result = execute("
111111
999911
999911
999911
999911
");
        assert_eq!(result, "9");
    }
}
