pub fn execute(input: &str) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    inputs.iter().map(solve).sum::<usize>().to_string()
}

fn solve(input: &Vec<Vec<char>>) -> usize {
    let result = findMirroredRow(input, 1);

    match result {
        Some(x) => x * 100,
        None => {
            let xpose = transpose(input);
            findMirroredRow(&xpose, 1).unwrap()
        },
    }
}

fn findMirroredRow(input: &Vec<Vec<char>>, num_errors: usize) -> Option<usize> {

    for x in 1..input.len() {
        let max_len = std::cmp::min(x, input.len() - x);
        let slice1 = &input[std::cmp::max(0, x - max_len)..x];
        let slice2 = &input[x..std::cmp::min(x + max_len, input.len())];

        if slice1.iter().zip(slice2.iter().rev())
                .map(|(s1, s2)| s1.iter().zip(s2.iter()).filter(|(c1, c2)| c1 != c2).count())
                .sum::<usize>() == num_errors {
            return Some(x);
        }
    }

    None
}

fn transpose(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i]).collect())
        .collect()
}

fn parseInput(input: &str) -> Vec<Vec<Vec<char>>> {

    input.split("\n\n").map(|x| x.split("\n").map(|x| x.chars().collect()).collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute("
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
");
        assert_eq!(result, "400");
    }
}
