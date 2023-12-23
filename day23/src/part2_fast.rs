use std::collections::{HashSet, HashMap, VecDeque};

use itertools::Itertools;
use rayon::iter::*;

pub fn execute(input: &str) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    let dirs:Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    let len = inputs.len();
    let mut nodes:HashMap<(isize, isize), Vec<(isize, isize)>> = (1..len-1).cartesian_product(1..len-1).filter_map(|(x,y)| {
        match inputs[y][x] {
            '#' => None,
            _ => {
                let map:Vec<(isize, isize)> = dirs.iter().filter_map(|dir| {
                    match inputs[(y as isize + dir.1) as usize][(x as isize + dir.0) as usize] {
                        '#' => None,
                        _ => Some(dir)
                    }
                }).map(|x| *x).collect();
                if map.len() > 2 {
                    Some(((x as isize,y as isize), map))
                } else {
                    None
                }
            }
        }
    }).collect();

    // insert the start and end nodes
    nodes.insert((1,0), vec![(0,1)]);
    nodes.insert((len as isize-2, len as isize-1), vec![(0,-1)]);

    let index_map:HashMap<(isize, isize), usize> = nodes.iter().enumerate().map(|(i, (pos, _))| {
        (*pos, i)
    }).collect();

    let mut edge_map:Vec<Vec<(usize, isize)>> = vec![vec![]; index_map.len()];

    nodes.iter().for_each(|(node, node_dirs)| {
        edge_map[*index_map.get(node).unwrap()] = node_dirs.iter().map(|dir| {
            let mut new_pos = (node.0 + dir.0, node.1 + dir.1);
            let mut visited:HashSet<(isize, isize)> = Default::default();
            let mut count:usize = 1;
            visited.insert(node.clone());
            loop {
                if nodes.contains_key(&new_pos) {
                    return (*index_map.get(&new_pos).unwrap(), count as isize);
                }
                visited.insert(new_pos.clone());
                new_pos = dirs.iter()
                    .map(|z| (new_pos.0 + z.0, new_pos.1 + z.1))
                    .filter(|z| inputs[z.1 as usize][z.0 as usize] != '#')
                    .filter(|z| !visited.contains(&z))
                    .collect::<Vec<(isize, isize)>>()[0];
                count += 1;
            }
        }).collect();
    });

    let start = *index_map.get(&(1,0)).unwrap();
    let end = *index_map.get(&(len as isize-2, len as isize-1)).unwrap();

    // let mut visited:Vec<bool> = vec![false; index_map.len()];
    // visited[start] = true;
    // visited[end] = true;

    // (longest(start, edge_map[end][0].0, &edge_map, &mut visited) + edge_map[end][0].1)
    //     .to_string()
    (longest_rayon(start, edge_map[end][0].0, &edge_map) + edge_map[end][0].1).to_string()
}

fn longest(start:usize, end: usize, map: &Vec<Vec<(usize, isize)>>, visited: &mut Vec<bool>) -> isize {
    let mut max = isize::MIN;
    for x in &map[start] {
        if visited[x.0] {
            continue;
        } else if x.0 == end {
            max = max.max(x.1);
        } else {
            visited[x.0] = true;
            max = max.max(longest(x.0, end, map, visited) + x.1);
            visited[x.0] = false;
        }
    }
    max
}

// fn longest_bfs(start:usize, end: usize, edge_map: &Vec<Vec<(usize, isize)>>) -> isize {
//     let mut queue: VecDeque<(usize, isize, Vec<bool>)> = Default::default();
//     let visited:Vec<bool> = vec![false; edge_map.len()];
//     let mut max:isize = 0;
//     queue.push_back((start, 0, visited));

//     while let Some((pos, len, mut visited)) = queue.pop_front() {
//         visited[pos] = true;
//         for x in &edge_map[pos] {
//             if visited[x.0] {
//                 continue;
//             } else if x.0 == end {
//                 max = max.max(len + x.1);
//             } else {
//                 queue.push_back((x.0, len + x.1, visited.clone()));
//             }
//         }
//     }

//     max
// }

fn longest_rayon(start:usize, end: usize, edge_map: &Vec<Vec<(usize, isize)>>) -> isize {
    let mut queue: VecDeque<(usize, isize, Vec<bool>)> = Default::default();
    let mut visited:Vec<bool> = vec![false; edge_map.len()];
    let mut max:isize = 0;
    visited[start] = true;
    queue.push_back((start, 0, visited));

    while let Some((pos, len, visited)) = queue.pop_front() {
        for x in &edge_map[pos] {
            if visited[x.0] {
                continue;
            } else if x.0 == end {
                max = max.max(len + x.1);
            } else {
                let mut new_visited = visited.clone();
                new_visited[x.0] = true;
                queue.push_back((x.0, len + x.1, new_visited));
            }
        }

        if queue.len() >= 16 {
            break;
        }
    }

    queue.par_iter().map(|(pos, len, visited)| {
        len + longest(*pos, end, edge_map, &mut visited.clone())
    }).max().unwrap_or(max)
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
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
");
        assert_eq!(result, "154");
    }
}
