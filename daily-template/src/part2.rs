use nom::{multi::separated_list1, character::complete::alphanumeric1, IResult, bytes::complete::tag};

pub fn execute(input: &str) -> String {
    let inputs = parseInput(input.trim()).unwrap();
    assert_eq!(inputs.0, "");
    let inputs = inputs.1;
    // println!("{:#?}", inputs);

    inputs[1].to_string()
}

fn parseInput(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag("\n"), alphanumeric1)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute("
a
b
");
        assert_eq!(result, "b");
    }
}
