pub fn execute(input: &str) -> String {
    let inputs = parseInput(input.trim());
    println!("{:#?}", inputs);

    inputs.iter().map(|x| solveLine(&x)).sum::<usize>().to_string()
}

fn parseInput(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    let lines = input.lines();

    lines.map(|line| {
        let split = line.split_once(" ").unwrap();
        (split.0.chars().collect() , split.1.split(",").map(|x| x.parse::<usize>().unwrap()).collect())
    }).collect()
}

fn solveLine(input: &(Vec<char>, Vec<usize>)) -> usize {

    let count = input.0.iter().filter(|x| **x == '?').count();
    let iterations = (2 as usize).pow(count as u32);
    let mut total = 0;

    for i in 0..iterations {

        let mut idx = 0;
        let mut cnt = 0;
        let mut sizes:Vec<usize> = Default::default();

        for j in 0..(input.0.len() + 1) {
            let mut ch = if j == input.0.len() {'.'} else {input.0[j]};
            if ch == '?' {
                if (i >> idx) & 1 == 1 {
                    ch = '.';
                } else {
                    ch = '#';
                }
                idx += 1;
            }

            match ch {
                '#' => {
                    cnt += 1;
                },
                '.' => {
                    if cnt != 0 {
                        sizes.push(cnt);
                        cnt = 0;
                    }
                },
                _ => panic!(),
            }
        }
        if sizes == input.1 {
            total += 1;
        }
    }

    total
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
        assert_eq!(result, "21");
    }
}
