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
}

impl Node {
    fn add_output(&mut self, s: &str) {
        match self {
            Node::Broadcaster(x) => x.outputs.push(s.to_string()),
            Node::FlipFlop(x) => x.outputs.push(s.to_string()),
            Node::Conjugation(x) => x.outputs.push(s.to_string()),
        }
    }

    fn add_input(&mut self, s: &str) {
        match self {
            Node::Broadcaster(_) => (),
            Node::FlipFlop(_) => (),
            Node::Conjugation(c) => {
                c.inputs.insert(s.to_string(), Pulse::Low);
            },
        }
    }
}

pub fn execute(input: &str) -> String {
    let mut nodes = parseInput(input.trim());
    // println!("{:#?}", nodes);

    let mut button_presses: usize = 0;
    let mut queue: VecDeque<(String, String, Pulse)> = Default::default();

    let mut hb_input_time: HashMap<String, usize> = Default::default();

    loop {
        button_presses += 1;

        queue.push_back(("broadcaster".to_string(), "button".to_string(), Pulse::Low));

        while let Some(next) = queue.pop_front() {

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
                    if next.2 == Pulse::High && next.0 == "hb" {
                        hb_input_time.insert(next.1.to_string(), button_presses);
                        if hb_input_time.len() == c.inputs.len() {
                            return hb_input_time.values().map(|x| *x).reduce(|x, y| num::integer::lcm(x, y)).unwrap().to_string()
                        }
                    }

                    *c.inputs.get_mut(&next.1).unwrap() = next.2;

                    let next_pulse = c.inputs.values().all(|x| *x == Pulse::High).then_some(Pulse::Low).unwrap_or(Pulse::High);

                    c.outputs.iter().for_each(|x| {
                        queue.push_back((x.to_string(), next.0.to_string(), next_pulse))
                    });
                },
            }
        }
    };
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
        outputs.split(", ").for_each(|output| {
            nodes.get_mut(&name).unwrap().add_output(output);
            nodes.get_mut(output).unwrap().add_input(&name);
        });
    });

    nodes
}
