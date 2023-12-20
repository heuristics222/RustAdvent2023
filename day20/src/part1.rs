use std::collections::{HashMap, VecDeque};


#[derive(Debug)]
struct Broadcaster {
    outputs: Vec<String>,
}

#[derive(Debug)]
struct FlipFlop {
    on: bool,
    outputs: Vec<String>,
}

#[derive(Debug)]
struct Conjugation {
    inputs: HashMap<String, Pulse>,
    outputs: Vec<String>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
enum Node {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjugation(Conjugation),
    Untyped(),
}

impl Node {
    fn add_output(&mut self, s: &str) {
        match self {
            Node::Broadcaster(x) => x.outputs.push(s.to_string()),
            Node::FlipFlop(x) => x.outputs.push(s.to_string()),
            Node::Conjugation(x) => x.outputs.push(s.to_string()),
            Node::Untyped() => (),
        }
    }

    fn add_input(&mut self, s: &str) {
        match self {
            Node::Broadcaster(_) => (),
            Node::FlipFlop(_) => (),
            Node::Conjugation(c) => {
                c.inputs.insert(s.to_string(), Pulse::Low);
            },
            Node::Untyped() => (),
        }
    }
}

pub fn execute(input: &str) -> String {
    let mut nodes = parseInput(input.trim());
    // println!("{:#?}", nodes);

    let mut low: usize = 0;
    let mut high: usize = 0;
    let mut queue: VecDeque<(String, String, Pulse)> = Default::default();

    (0..1000).for_each(|_| {

        queue.push_back(("broadcaster".to_string(), "button".to_string(), Pulse::Low));

        while let Some(next) = queue.pop_front() {
            match next.2 {
                Pulse::Low => low += 1,
                Pulse::High => high += 1,
            }

            let Some(ref mut node) = nodes.get_mut(&next.0) else { panic!() };

            match node {
                Node::Broadcaster(b) => {
                    b.outputs.iter().for_each(|x| {
                        queue.push_back((x.to_string(), next.0.to_string(), next.2))
                    });
                },
                Node::FlipFlop(ref mut f) => {
                    match next.2 {
                        Pulse::High => (),
                        Pulse::Low => {
                            f.on = !f.on;

                            let next_pulse = if f.on { Pulse::High } else { Pulse::Low };

                            f.outputs.iter().for_each(|x| {
                                queue.push_back((x.to_string(), next.0.to_string(), next_pulse))
                            });
                        },
                    }
                },
                Node::Conjugation(ref mut c) => {
                    *c.inputs.get_mut(&next.1).unwrap() = next.2;

                    let next_pulse = if c.inputs.values().all(|x| *x == Pulse::High) {Pulse::Low} else {Pulse::High};

                    c.outputs.iter().for_each(|x| {
                        queue.push_back((x.to_string(), next.0.to_string(), next_pulse))
                    });
                },
                Node::Untyped() => (),
            }
        }
    });


    (low * high).to_string()
}

fn parseInput(input: &str) -> HashMap<String, Node> {
    let lines:Vec<&str> = input.lines().collect();

    let mut nodes: HashMap<String, Node> = Default::default();

    lines.iter().for_each(|x| {
        let name = x.split_once(" -> ").unwrap().0;
        if name == "broadcaster" {
            nodes.insert("broadcaster".to_string(), Node::Broadcaster(Broadcaster{outputs:Vec::new()}));
        } else {
            match name.bytes().next().unwrap() {
                b'%' => nodes.insert(name[1..name.len()].to_string(), Node::FlipFlop(FlipFlop{on:false, outputs:Vec::new()})),
                b'&' => nodes.insert(name[1..name.len()].to_string(), Node::Conjugation(Conjugation{inputs:HashMap::new(), outputs:Vec::new()})),
                _ => panic!(),
            };
        }
    });

    lines.iter().for_each(|line| {
        let splits = line.split_once(" -> ").unwrap();
        let outputs = splits.1;
        let name = if splits.0 == "broadcaster" { &"broadcaster" } else { &splits.0[1..splits.0.len()] }.to_string();
        let mut node = nodes.remove(&name).unwrap();
        outputs.split(", ").for_each(|output| {
            node.add_output(output);
            let n = nodes.get(output);

            if n.is_none() {
                nodes.insert(output.to_string(), Node::Untyped());
            }

            nodes.get_mut(output).unwrap().add_input(&name);
        });
        nodes.insert(name.to_string(), node);
    });

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute("
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
");
        assert_eq!(result, "32000000");
    }

    #[test]
    fn it_works2() {
        let result = execute("
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
");
        assert_eq!(result, "11687500");
    }
}
