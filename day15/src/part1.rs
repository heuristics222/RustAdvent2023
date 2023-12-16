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

pub fn execute_fast(input: &str) -> String {
    let mut hash_sum = 0u32;
    let mut cur_hash = 0u8;

    input.trim().bytes().for_each(|b| {
        match b {
            b',' => {
                hash_sum += cur_hash as u32;
                cur_hash = 0;
            }
            _ => cur_hash = cur_hash.wrapping_add(b).wrapping_mul(17),
        }
    });

    hash_sum.to_string()
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
