use std::collections::{HashMap, VecDeque};

mod parsing;

#[derive(Clone, Copy, Debug, PartialEq)]
enum PulseType {
    Low,
    High,
}

#[derive(Clone, Debug, PartialEq)]
struct Pulse {
    src: String,
    typ: PulseType,
    dst: String,
}

#[derive(Clone, Debug, PartialEq)]
enum Module {
    Broadcaster {
        name: String,
        outputs: Vec<String>,
    },
    FlipFlop {
        name: String,
        is_on: bool,
        outputs: Vec<String>,
    },
    Conj {
        name: String,
        inputs: HashMap<String, PulseType>,
        outputs: Vec<String>,
    },
    Sink {
        dummy: Vec<String>,
    },
}

impl Module {
    fn handle(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        match self {
            Module::Broadcaster { name, outputs } => handle_broadcaster(name, outputs, pulse),
            Module::FlipFlop {
                name,
                is_on,
                outputs,
            } => handle_flipflop(name, is_on, outputs, pulse),
            Module::Conj {
                name,
                inputs,
                outputs,
            } => handle_conj(name, inputs, outputs, pulse),
            Module::Sink { dummy: _ } => vec![],
        }
    }

    fn get_outputs(&self) -> &[String] {
        match self {
            Module::Broadcaster { outputs, .. } => outputs,
            Module::FlipFlop { outputs, .. } => outputs,
            Module::Conj { outputs, .. } => outputs,
            Module::Sink { dummy } => dummy,
        }
    }
}

fn handle_broadcaster(name: &str, outputs: &[String], pulse: &Pulse) -> Vec<Pulse> {
    outputs
        .iter()
        .map(|output| Pulse {
            typ: pulse.typ,
            src: name.to_string(),
            dst: output.to_string(),
        })
        .collect()
}

fn handle_flipflop(name: &str, is_on: &mut bool, outputs: &[String], pulse: &Pulse) -> Vec<Pulse> {
    if pulse.typ == PulseType::High {
        vec![]
    } else {
        *is_on = !*is_on;
        outputs
            .iter()
            .map(|output| Pulse {
                typ: if *is_on {
                    PulseType::High
                } else {
                    PulseType::Low
                },
                src: name.to_string(),
                dst: output.to_string(),
            })
            .collect()
    }
}

fn handle_conj(
    name: &str,
    inputs: &mut HashMap<String, PulseType>,
    outputs: &[String],
    pulse: &Pulse,
) -> Vec<Pulse> {
    *inputs.get_mut(&pulse.src).unwrap() = pulse.typ;

    outputs
        .iter()
        .map(|output| Pulse {
            typ: if inputs.values().all(|val| *val == PulseType::High) {
                PulseType::Low
            } else {
                PulseType::High
            },
            src: name.to_string(),
            dst: output.clone(),
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq)]
struct Conf {
    modules: HashMap<String, Module>,
}

struct Solver;

impl aoc::Solver for Solver {
    type Input = Conf;
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        parsing::parse(input)
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut config = input.clone();

        let mut low_pulses = 0;
        let mut high_pulses = 0;

        for _ in 0..1000 {
            let mut pulses = VecDeque::from([Pulse {
                src: "button".to_string(),
                typ: PulseType::Low,
                dst: "broadcaster".to_string(),
            }]);

            while let Some(pulse) = pulses.pop_front() {
                match pulse.typ {
                    PulseType::Low => low_pulses += 1,
                    PulseType::High => high_pulses += 1,
                }

                pulses.extend(
                    config
                        .modules
                        .get_mut(&pulse.dst)
                        .unwrap()
                        .handle(&pulse)
                        .into_iter(),
                );
            }
        }

        low_pulses * high_pulses
    }

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        todo!()
    }
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        Conf {
            modules: [
                (
                    "broadcaster".to_string(),
                    Module::Broadcaster {
                        name: "broadcaster".to_string(),
                        outputs: vec!["a".to_string()],
                    },
                ),
                (
                    "a".to_string(),
                    Module::FlipFlop {
                        name: "a".to_string(),
                        is_on: false,
                        outputs: vec!["inv".to_string(), "con".to_string()],
                    },
                ),
                (
                    "inv".to_string(),
                    Module::Conj {
                        name: "inv".to_string(),
                        inputs: [("a".to_string(), PulseType::Low)].into(),
                        outputs: vec!["b".to_string()],
                    },
                ),
                (
                    "b".to_string(),
                    Module::FlipFlop {
                        name: "b".to_string(),
                        is_on: false,
                        outputs: vec!["con".to_string()],
                    },
                ),
                (
                    "con".to_string(),
                    Module::Conj {
                        name: "con".to_string(),
                        inputs: [
                            ("a".to_string(), PulseType::Low),
                            ("b".to_string(), PulseType::Low),
                        ]
                        .into(),
                        outputs: vec!["output".to_string()],
                    },
                ),
                ("output".to_string(), Module::Sink { dummy: vec![] }),
            ]
            .into_iter()
            .collect(),
        }
    }

    #[test]
    fn parsing() {
        let input = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 11687500);
    }
}
