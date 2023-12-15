pub fn execute(input: &str) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    let v:Vec<(&str, &str)> = Default::default();
    let mut boxes = vec![v; 256];

    inputs.iter().for_each(|x| {
        let split = if x.contains("=") {
            x.split_once("=")
        } else {
            x.split_once("-")
        }.unwrap();
        let label = split.0;
        let b = hash(split.0) as usize;
        let focal = split.1;

        if x.contains("=") {
            if let Some(idx) = boxes[b].iter().position(|v| v.0 == label) {
                boxes[b][idx] = (label, focal);
            } else {
                boxes[b].push((label, focal));
            }
        } else {
            if let Some(idx) = boxes[b].iter().position(|v| v.0 == label) {
                boxes[b].remove(idx);
            }
        }
    });

    boxes.iter().enumerate().map(|(box_num, v)| {
        v.iter().enumerate().map(|(slot, (_label, focal))| {
            (box_num + 1) * (slot + 1) * focal.parse::<usize>().unwrap()
        }).sum::<usize>()
    }).sum::<usize>().to_string()
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
        assert_eq!(result, "145");
    }
}
