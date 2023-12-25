use std::collections::{HashSet, BTreeMap};

pub fn execute(input: &str) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    let g = inputs.clone();

    let partition = min_cut(&g);
    (partition.len() * (inputs.len() - partition.len())).to_string()
}

pub fn min_cut(inputs: &Vec<Vec<usize>>) -> HashSet<usize> {
    let mut g = inputs.clone();

    let mut best_partition:HashSet<usize> = Default::default();
    let mut partitions:Vec<HashSet<usize>> = (0..inputs.len()).map(|x| HashSet::from_iter(vec![x].iter().map(|z| *z))).collect();
    let mut w_min = usize::MAX;

    let mut len = g.iter().map(|x| x.iter().any(|y| *y != 0)).filter(|b| *b).count();

    while len > 1 {
        let (s, t, w) = max_adj_search(&g, None);

        let t_part = partitions[t].clone();
        partitions[s].extend(t_part.iter());

        if w < w_min {
            w_min = w;
            best_partition.clear();
            best_partition.extend(partitions[t].iter());
        }

        merge_nodes(&mut g, s, t);

        len -= 1;
    }

    if best_partition.contains(&0) {
        best_partition
    } else {
        get_all_nodes(&inputs).difference(&best_partition).map(|x| *x).collect()
    }
}

// merge t with s
pub fn merge_nodes(g: &mut Vec<Vec<usize>>, s: usize, t: usize) {
    // Remove edge s-t
    g[t][s] = 0;
    g[s][t] = 0;

    // Merge t's remaining edges with s's
    for i in 0..g.len() {
        g[s][i] += g[t][i];
        g[i][s] += g[t][i];

        g[t][i] = 0;
        g[i][t] = 0;
    }
}

fn get_all_nodes(g: &Vec<Vec<usize>>) -> HashSet<usize> {
    g.iter().enumerate().filter_map(|(i, edges)| {
        if edges.iter().any(|x| *x != 0) {
            Some(i)
        } else {
            None
        }
    }).collect()
}

pub fn max_adj_search<'a>(g: &Vec<Vec<usize>>, s: Option<usize>) -> (usize, usize, usize) {
    let mut candidates: HashSet<usize> = get_all_nodes(g);
    let start = s.map_or_else(|| *candidates.iter().next().unwrap(), |x| x);
    let mut found_set: Vec<usize> = vec![start];
    let mut cut_weight: Vec<usize> = Default::default();

    candidates.remove(&start);

    while !candidates.is_empty() {
        let mut max_next_node = usize::MAX;
        let mut max_weight = 0;

        candidates.iter().for_each(|next| {
            let mut weight_sum = 0;

            found_set.iter().for_each(|s| {
                if g[*next][*s] != 0 {
                    weight_sum += g[*next][*s];
                }
            });

            if weight_sum > max_weight {
                max_next_node = *next;
                max_weight = weight_sum;
            }
        });

        candidates.remove(&max_next_node);
        found_set.push(max_next_node);
        cut_weight.push(max_weight);
    }

    (
        *found_set.get(found_set.len() - 2).unwrap(),
        *found_set.last().unwrap(),
        *cut_weight.last().unwrap()
    )
}

fn parseInput(input: &str) -> Vec<Vec<usize>> {
    let mut indexes:BTreeMap<&str, usize> = Default::default();
    let mut i = 0;
    let x:Vec<Vec<&str>> = input.lines().map(|line| {
        line.split(&[':', ' '])
            .filter(|x| !x.is_empty())
            .inspect(|x| {
                match indexes.get(*x) {
                    None => {
                        indexes.insert(*x, i);
                        i += 1;
                    },
                    _ => (),
                }
            }).collect()
    }).collect();

    // let mut m:HashMap<&str, HashSet<&str>> = Default::default();
    let mut weights:Vec<Vec<usize>> = vec![vec![0; indexes.len()]; indexes.len()];

    x.iter().for_each(|set| {
        let st = set[0];
        set[1..].iter().for_each(|end| {
            weights[*indexes.get(st).unwrap()][*indexes.get(end).unwrap()] = 1;
            weights[*indexes.get(end).unwrap()][*indexes.get(st).unwrap()] = 1;
        });
    });

    weights
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute("
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
");
        assert_eq!(result, "54");
    }

    #[test]
    fn it_works2() {
        let g = get_test_input();
        let result = min_cut(&g);

        assert_eq!(result, HashSet::from_iter(vec![0, 1, 4, 5].iter().map(|x| *x)));
    }

    fn get_test_input() -> Vec<Vec<usize>> {
        let mut g = vec![vec![0; 8]; 8];
        g[0][1] = 2;
        g[1][0] = 2;

        g[0][4] = 3;
        g[4][0] = 3;

        g[4][1] = 2;
        g[1][4] = 2;

        g[5][1] = 2;
        g[1][5] = 2;

        g[5][4] = 3;
        g[4][5] = 3;

        g[1][2] = 3;
        g[2][1] = 3;

        g[5][6] = 1;
        g[6][5] = 1;

        g[2][6] = 2;
        g[6][2] = 2;

        g[2][3] = 4;
        g[3][2] = 4;

        g[3][6] = 2;
        g[6][3] = 2;

        g[7][6] = 3;
        g[6][7] = 3;

        g[7][3] = 2;
        g[3][7] = 2;

        g
    }

    #[test]
    fn max_adj() {
        let result = max_adj_search(&get_test_input(), Some(1));

        assert_eq!(result, (4, 0, 5))
    }

    #[test]
    fn merge1() {
        let mut g = get_test_input();
        merge_nodes(&mut g, 4, 0);

        let mut expected = vec![vec![0; 8]; 8];
        expected[4][1] = 4;
        expected[1][4] = 4;

        expected[5][1] = 2;
        expected[1][5] = 2;

        expected[5][4] = 3;
        expected[4][5] = 3;

        expected[1][2] = 3;
        expected[2][1] = 3;

        expected[5][6] = 1;
        expected[6][5] = 1;

        expected[2][6] = 2;
        expected[6][2] = 2;

        expected[2][3] = 4;
        expected[3][2] = 4;

        expected[3][6] = 2;
        expected[6][3] = 2;

        expected[7][6] = 3;
        expected[6][7] = 3;

        expected[7][3] = 2;
        expected[3][7] = 2;

        assert_eq!(g, expected);
    }

    #[test]
    fn merge2() {
        let mut g = vec![vec![0; 8]; 8];
        g[2][1] = 3;
        g[1][2] = 3;

        g[2][6] = 2;
        g[6][2] = 2;

        g[1][5] = 2;
        g[5][1] = 2;

        g[6][5] = 1;
        g[5][6] = 1;

        merge_nodes(&mut g, 6, 2);

        let mut expected = vec![vec![0; 8]; 8];
        expected[1][5] = 2;
        expected[5][1] = 2;

        expected[6][5] = 1;
        expected[5][6] = 1;

        expected[6][1] = 3;
        expected[1][6] = 3;

        assert_eq!(g, expected);
    }
}
