use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    xmas: char,
    op: char,
    val: usize,
    target: String,
}

#[derive(Clone)]
struct Ranges {
    min_x: usize,
    min_m: usize,
    min_a: usize,
    min_s: usize,
    max_x: usize,
    max_m: usize,
    max_a: usize,
    max_s: usize,
}

impl Ranges {
    pub fn new() -> Self {
        Self {min_x: 1, min_m: 1, min_a: 1, min_s: 1, max_x: 4001, max_m: 4001, max_a: 4001, max_s: 4001 }
    }

    fn product(&self) -> usize {
        (self.max_x - self.min_x).max(0) * (self.max_m - self.min_m).max(0) * (self.max_a - self.min_a).max(0) * (self.max_s - self.min_s).max(0)
    }
}

pub fn execute(input: &str) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    let part:Ranges = Ranges::new();
    let instr_name = "in".to_string();

    solve(&part, &inputs, &instr_name).to_string()
}

fn solve(ranges: &Ranges, instructions: &HashMap<String, Vec<Rule>>, instr_name: &String) -> usize {
    if instr_name == "A" {
        return ranges.product();
    } else if instr_name == "R" {
        return 0;
    }
    let mut temp_ranges = ranges.clone();

    instructions.get(instr_name).unwrap().iter().map(|x| {
        let mut new_ranges = temp_ranges.clone();
        match (x.xmas, x.op) {
            ('x', '<') => { new_ranges.max_x = x.val.min(new_ranges.max_x); temp_ranges.min_x = (x.val).max(temp_ranges.min_x);},
            ('m', '<') => { new_ranges.max_m = x.val.min(new_ranges.max_m); temp_ranges.min_m = (x.val).max(temp_ranges.min_m);},
            ('a', '<') => { new_ranges.max_a = x.val.min(new_ranges.max_a); temp_ranges.min_a = (x.val).max(temp_ranges.min_a);},
            ('s', '<') => { new_ranges.max_s = x.val.min(new_ranges.max_s); temp_ranges.min_s = (x.val).max(temp_ranges.min_s);},
            ('x', '>') => { new_ranges.min_x = (x.val + 1).max(new_ranges.min_x); temp_ranges.max_x = (x.val + 1).min(temp_ranges.max_x);},
            ('m', '>') => { new_ranges.min_m = (x.val + 1).max(new_ranges.min_m); temp_ranges.max_m = (x.val + 1).min(temp_ranges.max_m);},
            ('a', '>') => { new_ranges.min_a = (x.val + 1).max(new_ranges.min_a); temp_ranges.max_a = (x.val + 1).min(temp_ranges.max_a);},
            ('s', '>') => { new_ranges.min_s = (x.val + 1).max(new_ranges.min_s); temp_ranges.max_s = (x.val + 1).min(temp_ranges.max_s);},
            _ => panic!(),
        }
        solve(&new_ranges, instructions, &x.target)
    }).sum()
}

fn parseInput(input: &str) -> HashMap<String, Vec<Rule>> {
    let lines = input.split_once("\n\n").unwrap();

    let instructions:HashMap<String, Vec<Rule>> = lines.0.lines().map(|instr| {
        let s = instr.split_once("{").unwrap();
        let rules = &s.1[0..s.1.len()-1];
        let rules = rules.split(",").map(|x| {
            if x.contains(":") {
                let split = x.split_once(":").unwrap();
                let v = &split.0[0..1];
                let val = split.0[2..split.0.len()].parse().unwrap();
                let op = &split.0[1..2];

                Rule {xmas:v.chars().nth(0).unwrap(), op:op.chars().nth(0).unwrap(), val, target:split.1.to_string()}
            } else {
                Rule {xmas:'x', op:'<', val: 10000, target:x.to_string()}
            }
        }).collect();

        (s.0.to_string(), rules)
    }).collect();


    instructions
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
        assert_eq!(result, "167409079868000");
    }
}
