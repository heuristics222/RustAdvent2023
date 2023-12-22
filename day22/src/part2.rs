use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
struct Block {
    index: usize,
    min_x: usize,
    min_y: usize,
    min_z: usize,
    max_x: usize,
    max_y: usize,
    max_z: usize,
}

impl Block {
    fn move_down(&mut self, amt: usize) {
        self.min_z -= amt;
        self.max_z -= amt;
    }

    fn col_intersect(&self, other: &Block) -> bool {
        other.max_x >= self.min_x && other.min_x <= self.max_x &&
        other.max_y >= self.min_y && other.min_y <= self.max_y
    }
}

pub fn execute(input: &str) -> String {
    let mut inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    inputs.sort_by(|x, y| {
        if x.min_z < y.min_z {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    settle_blocks(&mut inputs);

    (0..inputs.len()).map(|i| {
        let mut new_inputs = inputs.iter().filter(|b| b.index != i).map(|x| x.clone()).collect_vec();
        print!("{:#?}", inputs[i]);
        settle_blocks(&mut new_inputs)
    }).inspect(|f| println!("{f}")).sum::<usize>().to_string()
}

fn settle_blocks(blocks: &mut Vec<Block>) -> usize {

    let mut count = 0;

    for idx in 0..blocks.len() {
        let block = blocks.get(idx).unwrap().clone();
        let block_under_z = blocks[0..idx].iter().filter(|x| block.col_intersect(x)).max_by(|x, y| x.max_z.cmp(&y.max_z)).map_or(0, |x| x.max_z);
        let amt = block.min_z - block_under_z - 1;
        if amt > 0 {
            blocks.get_mut(idx).unwrap().move_down(amt);
            count += 1;
        }
    }

    count
}

fn parseInput(input: &str) -> Vec<Block> {
    input.lines().enumerate().map(|(index, line)| {
        let split = line.split_once("~").unwrap();
        let coord1:Vec<usize> = split.0.split(",").map(|x| x.parse().unwrap()).collect();
        let coord2:Vec<usize> = split.1.split(",").map(|x| x.parse().unwrap()).collect();
        let min_x = coord1[0].min(coord2[0]);
        let min_y = coord1[1].min(coord2[1]);
        let min_z = coord1[2].min(coord2[2]);
        let max_x = coord1[0].max(coord2[0]);
        let max_y = coord1[1].max(coord2[1]);
        let max_z = coord1[2].max(coord2[2]);

        Block{index, min_x, min_y, min_z, max_x, max_y, max_z}
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute("
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
");
        assert_eq!(result, "7");
    }
}
