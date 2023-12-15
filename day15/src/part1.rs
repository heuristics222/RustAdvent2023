pub fn execute(input: &str) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    inputs.iter().map(|x| hash(x)).sum::<u32>().to_string()
}

fn hash(input: &str) -> u32 {
    let mut hash = 0;
    input.chars().for_each(|x| {
        hash = ((hash + x as u32) * 17) % 256;
    });

    hash
}

fn parseInput(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute("
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
");
        assert_eq!(result, "1320");
    }
}
