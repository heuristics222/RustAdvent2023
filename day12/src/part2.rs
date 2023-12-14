use std::{iter::repeat, collections::HashMap};

pub fn execute(input: &str) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    let combos:Vec<usize> = inputs.iter().map(|x| solveLine(&x)).collect();
    dbg!(combos).iter().sum::<usize>().to_string()
}

fn parseInput(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    let lines = input.lines();

    lines.map(|line| {
        let split = line.split_once(" ").unwrap();

        (vec![split.0; 5].join("?").chars().collect(), repeat(split.1.split(",").map(|x| x.parse::<usize>().unwrap())).take(5).flatten().collect())
    }).collect()
}

fn solveLine(input: &(Vec<char>, Vec<usize>)) -> usize {
    let mut cache:HashMap<(usize, usize, usize), usize> = HashMap::new();

    let solution = next(&mut cache, &input.0[..], &input.1[..], 0);

    println!("{solution}");
    solution
}

fn next(cache: &mut HashMap<(usize, usize, usize), usize>, line: &[char], remainingSizes: &[usize], numHash: usize) -> usize {

    if line.len() == 0 {
        if numHash == 0 && remainingSizes.len() == 0 {
            return 1;
        }
        if remainingSizes.len() == 1 && remainingSizes[0] == numHash {
            return 1;
        }
        return 0;
    }

    let key = (line.len(), remainingSizes.len(), numHash);
    if let Some(&x) = cache.get(&key) {
        return x;
    }

    let ch = line[0];

    let mut val = 0;

    if ch == '?' || ch == '.' {
        if numHash > 0 {
            if numHash != remainingSizes[0] {
                val += 0
            } else {
                val += next(cache, &line[1..], &remainingSizes[1..], 0)
            }
        } else {
            val += next(cache, &line[1..], remainingSizes, 0)
        }
    }
    if ch == '?' || ch == '#' {
        if remainingSizes.len() == 0 {
            val += 0;
        } else {
            val += next(cache, &line[1..], remainingSizes, numHash + 1)
        }
    }

    cache.insert(key, val);

    val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute("
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
");
        assert_eq!(result, "525152");
    }
}
