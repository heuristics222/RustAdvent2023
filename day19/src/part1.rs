use std::collections::HashMap;

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug)]
struct Instruction {
    rules: Vec<String>,
}

impl Part {
    fn sum(&self) -> usize {
        return self.x + self.m + self.a + self.s;
    }
}

pub fn execute(input: &str) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    inputs.1.iter().map(|part| {
        let mut instr_name = "in".to_string();

        loop {
            if instr_name == "A" {
                return part.sum();
            } else if instr_name == "R" {
                return 0;
            }

            let instr = inputs.0.get(&instr_name).unwrap();
            for x in &instr.rules {
                if x.contains(":") {
                    let split = x.split_once(":").unwrap();
                    let v = &split.0[0..1];
                    let part_v = match v {
                        "x" => part.x,
                        "m" => part.m,
                        "a" => part.a,
                        "s" => part.s,
                        _ => panic!(),
                    };
                    let val = split.0[2..split.0.len()].parse().unwrap();
                    let op = &split.0[1..2];
                    let matches = match op {
                        "<" => part_v < val,
                        ">" => part_v > val,
                        _ => panic!(),
                    };

                    if matches {
                        instr_name = split.1.to_string();
                        break;
                    }
                } else {
                    instr_name = x.to_string();
                }
            }
        }
    }).sum::<usize>().to_string()
}

fn parseInput(input: &str) -> (HashMap<String, Instruction>, Vec<Part>) {
    let split = input.split_once("\n\n").unwrap();

    let instructions:HashMap<String, Instruction> = split.0.lines().map(|instr| {
        let s = instr.split_once("{").unwrap();
        let rules = &s.1[0..s.1.len()-1];

        let i = Instruction {rules:rules.split(",").map(|x| x.to_string()).collect()};

        (s.0.to_string(), i)
    }).collect();

    let parts:Vec<Part> = split.1.lines().map(|line| {
        let line = &line[1..line.len()-1];
        let s:Vec<usize> = line.split(",").map(|x| x[2..x.len()].parse().unwrap()).collect();

        Part {x:s[0], m:s[1], a:s[2], s:s[3]}
    }).collect();

    (instructions, parts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute("
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
");
        assert_eq!(result, "19114");
    }
}
