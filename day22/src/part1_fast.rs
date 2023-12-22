use std::collections::HashSet;

use itertools::Itertools;

struct Block {
    min_x: usize,
    min_y: usize,
    min_z: usize,
    max_x: usize,
    max_y: usize,
    max_z: usize,
}

impl Block {
    fn set_min(&mut self, new_min: usize) {
        self.max_z = new_min + (self.max_z - self.min_z);
        self.min_z = new_min;
    }
}

pub fn execute(input: &str) -> String {
    let mut blocks = parseInput(input.trim());
    // println!("{:#?}", blocks);

    let mut supports: Vec<HashSet<usize>> = vec![Default::default(); blocks.len()];
    let mut supported_bys: Vec<HashSet<usize>> = vec![Default::default(); blocks.len()];
    settle_blocks(&mut blocks, &mut supports, &mut supported_bys);

    supports.iter().enumerate().filter(|(_, supported)| {
        supported.iter().all(|idx| {
            supported_bys[*idx].len() > 1
        })
    }).count().to_string()
}

fn settle_blocks(blocks: &mut Vec<Block>, supports: &mut Vec<HashSet<usize>>, supported_by: &mut Vec<HashSet<usize>>) {
    blocks.sort_by(|x, y| {
        x.min_z.cmp(&y.min_z)
    });

    // Grid is at most 10x10
    let mut heights:Vec<usize> = vec![0; 100];
    let mut indexes:Vec<usize> = vec![usize::MAX; 100];

    blocks.iter_mut().enumerate().for_each(|(i,b)| {
        let z = (b.min_x..=b.max_x).cartesian_product(b.min_y..=b.max_y).map(|(x,y)| {
            heights[y * 10 + x]
        }).max().unwrap();
        b.set_min(z + 1);

        (b.min_x..=b.max_x).cartesian_product(b.min_y..=b.max_y).for_each(|(x,y)| {
            let supporting_block_index = indexes[y * 10 + x];
            let prev_height = heights[y * 10 + x];
            heights[y * 10 + x] = b.max_z;
            indexes[y * 10 + x] = i;
            if supporting_block_index != usize::MAX && prev_height == b.min_z - 1 {
                supported_by[i].insert(supporting_block_index);
                supports[supporting_block_index].insert(i);
            }
        });
    });
}

fn parseInput(input: &str) -> Vec<Block> {
    input.lines().map(|line| {
        let split = line.split_once("~").unwrap();
        let coord1:Vec<usize> = split.0.split(",").map(|x| x.parse().unwrap()).collect();
        let coord2:Vec<usize> = split.1.split(",").map(|x| x.parse().unwrap()).collect();
        let min_x = coord1[0].min(coord2[0]);
        let min_y = coord1[1].min(coord2[1]);
        let min_z = coord1[2].min(coord2[2]);
        let max_x = coord1[0].max(coord2[0]);
        let max_y = coord1[1].max(coord2[1]);
        let max_z = coord1[2].max(coord2[2]);

        Block{min_x, min_y, min_z, max_x, max_y, max_z}
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
        assert_eq!(result, "5");
    }
}
