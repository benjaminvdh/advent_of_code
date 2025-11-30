use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::str::FromStr;

struct Solver;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Op {
    Gt,
    Lt,
}

impl From<char> for Op {
    fn from(c: char) -> Self {
        match c {
            '>' => Op::Gt,
            '<' => Op::Lt,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Prop {
    X,
    M,
    A,
    S,
}

impl From<char> for Prop {
    fn from(c: char) -> Self {
        match c {
            'x' => Prop::X,
            'm' => Prop::M,
            'a' => Prop::A,
            's' => Prop::S,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Out {
    Cont(String),
    Ignore,
    Accept,
    Reject,
}

impl From<&str> for Out {
    fn from(string: &str) -> Self {
        if string == "A" {
            Out::Accept
        } else if string == "R" {
            Out::Reject
        } else {
            Out::Cont(string.to_string())
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Cond {
    Test {
        prop: Prop,
        op: Op,
        crit: usize,
        out: Out,
    },
    Always(Out),
}

impl Cond {
    pub fn apply(&self, part: &Part) -> Out {
        match self {
            Cond::Test {
                prop,
                op,
                crit,
                out,
            } => {
                if match op {
                    Op::Gt => match prop {
                        Prop::X => part.x > *crit,
                        Prop::M => part.m > *crit,
                        Prop::A => part.a > *crit,
                        Prop::S => part.s > *crit,
                    },
                    Op::Lt => match prop {
                        Prop::X => part.x < *crit,
                        Prop::M => part.m < *crit,
                        Prop::A => part.a < *crit,
                        Prop::S => part.s < *crit,
                    },
                } {
                    out.clone()
                } else {
                    Out::Ignore
                }
            }
            Cond::Always(out) => out.clone(),
        }
    }

    pub fn split(&self, multipart: &Multipart) -> (Option<(Multipart, Out)>, Option<Multipart>) {
        match self {
            Cond::Test {
                prop,
                op,
                crit,
                out,
            } => match op {
                Op::Gt => match prop {
                    Prop::X => {
                        let acc = crit + 1..=*multipart.x.end();
                        let acc = if acc.is_empty() {
                            None
                        } else {
                            Some((
                                Multipart {
                                    x: acc,
                                    ..multipart.clone()
                                },
                                out.clone(),
                            ))
                        };

                        let rej = *multipart.x.start()..=*crit;
                        let rej = if rej.is_empty() {
                            None
                        } else {
                            Some(Multipart {
                                x: rej,
                                ..multipart.clone()
                            })
                        };

                        (acc, rej)
                    }
                    Prop::M => {
                        let acc = crit + 1..=*multipart.m.end();
                        let acc = if acc.is_empty() {
                            None
                        } else {
                            Some((
                                Multipart {
                                    m: acc,
                                    ..multipart.clone()
                                },
                                out.clone(),
                            ))
                        };

                        let rej = *multipart.m.start()..=*crit;
                        let rej = if rej.is_empty() {
                            None
                        } else {
                            Some(Multipart {
                                m: rej,
                                ..multipart.clone()
                            })
                        };

                        (acc, rej)
                    }
                    Prop::A => {
                        let acc = crit + 1..=*multipart.a.end();
                        let acc = if acc.is_empty() {
                            None
                        } else {
                            Some((
                                Multipart {
                                    a: acc,
                                    ..multipart.clone()
                                },
                                out.clone(),
                            ))
                        };

                        let rej = *multipart.a.start()..=*crit;
                        let rej = if rej.is_empty() {
                            None
                        } else {
                            Some(Multipart {
                                a: rej,
                                ..multipart.clone()
                            })
                        };

                        (acc, rej)
                    }
                    Prop::S => {
                        let acc = crit + 1..=*multipart.s.end();
                        let acc = if acc.is_empty() {
                            None
                        } else {
                            Some((
                                Multipart {
                                    s: acc,
                                    ..multipart.clone()
                                },
                                out.clone(),
                            ))
                        };

                        let rej = *multipart.s.start()..=*crit;
                        let rej = if rej.is_empty() {
                            None
                        } else {
                            Some(Multipart {
                                s: rej,
                                ..multipart.clone()
                            })
                        };

                        (acc, rej)
                    }
                },
                Op::Lt => match prop {
                    Prop::X => {
                        let acc = *multipart.x.start()..=crit - 1;
                        let acc = if acc.is_empty() {
                            None
                        } else {
                            Some((
                                Multipart {
                                    x: acc,
                                    ..multipart.clone()
                                },
                                out.clone(),
                            ))
                        };

                        let rej = *crit..=*multipart.x.end();
                        let rej = if rej.is_empty() {
                            None
                        } else {
                            Some(Multipart {
                                x: rej,
                                ..multipart.clone()
                            })
                        };

                        (acc, rej)
                    }
                    Prop::M => {
                        let acc = *multipart.m.start()..=crit - 1;
                        let acc = if acc.is_empty() {
                            None
                        } else {
                            Some((
                                Multipart {
                                    m: acc,
                                    ..multipart.clone()
                                },
                                out.clone(),
                            ))
                        };

                        let rej = *crit..=*multipart.m.end();
                        let rej = if rej.is_empty() {
                            None
                        } else {
                            Some(Multipart {
                                m: rej,
                                ..multipart.clone()
                            })
                        };

                        (acc, rej)
                    }
                    Prop::A => {
                        let acc = *multipart.a.start()..=crit - 1;
                        let acc = if acc.is_empty() {
                            None
                        } else {
                            Some((
                                Multipart {
                                    a: acc,
                                    ..multipart.clone()
                                },
                                out.clone(),
                            ))
                        };

                        let rej = *crit..=*multipart.a.end();
                        let rej = if rej.is_empty() {
                            None
                        } else {
                            Some(Multipart {
                                a: rej,
                                ..multipart.clone()
                            })
                        };

                        (acc, rej)
                    }
                    Prop::S => {
                        let acc = *multipart.s.start()..=crit - 1;
                        let acc = if acc.is_empty() {
                            None
                        } else {
                            Some((
                                Multipart {
                                    s: acc,
                                    ..multipart.clone()
                                },
                                out.clone(),
                            ))
                        };

                        let rej = *crit..=*multipart.s.end();
                        let rej = if rej.is_empty() {
                            None
                        } else {
                            Some(Multipart {
                                s: rej,
                                ..multipart.clone()
                            })
                        };

                        (acc, rej)
                    }
                },
            },
            Cond::Always(out) => (Some((multipart.clone(), out.clone())), None),
        }
    }
}

impl From<&str> for Cond {
    fn from(input: &str) -> Cond {
        if let Some(c) = input.chars().nth(1) {
            if c == '>' || c == '<' {
                let colon = input.find(':').unwrap();
                return Cond::Test {
                    prop: Prop::from(input.chars().nth(0).unwrap()),
                    op: Op::from(input.chars().nth(1).unwrap()),
                    crit: usize::from_str(&input[2..colon]).unwrap(),
                    out: Out::from(&input[colon + 1..]),
                };
            }
        }

        Cond::Always(Out::from(input))
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Workflow {
    conds: Vec<Cond>,
}

impl Workflow {
    pub fn apply(&self, part: &Part) -> Out {
        for cond in &self.conds {
            match cond.apply(part) {
                o @ Out::Cont(_) | o @ Out::Accept | o @ Out::Reject => {
                    return o;
                }
                Out::Ignore => continue,
            }
        }
        panic!()
    }

    pub fn apply_multipart(&self, multipart: &Multipart) -> Vec<(Multipart, Out)> {
        let mut multiparts = vec![];
        let mut multipart = multipart.clone();

        for cond in &self.conds {
            let (acc, rej) = cond.split(&multipart);

            if let Some((acc, out)) = acc {
                multiparts.push((acc, out));
            }

            if let Some(rej) = rej {
                multipart = rej;
            }

            if multipart.is_empty() {
                break;
            }
        }

        multiparts
    }
}

fn parse_workflow(line: &str) -> (String, Workflow) {
    let (name, workflow) = line.split_once('{').unwrap();
    let workflow = &workflow[..workflow.len() - 1];
    let conds = workflow
        .split(',')
        .map(|condition| condition.into())
        .collect();
    (name.to_string(), Workflow { conds })
}

#[derive(Debug, PartialEq)]
struct Workflows(HashMap<String, Workflow>);

impl Workflows {
    pub fn accepts(&self, part: &Part) -> bool {
        let mut cur_name = "in".to_string();

        loop {
            match self.0.get(&cur_name).unwrap().apply(part) {
                Out::Accept => return true,
                Out::Reject => return false,
                Out::Cont(next) => cur_name = next,
                _ => panic!(),
            }
        }
    }

    pub fn get_num_accepted_ratings(&self) -> usize {
        let mut tentative = vec![(Multipart::new(), Out::Cont("in".to_string()))];
        let mut accepted = vec![];

        loop {
            let mut new_tentative = vec![];

            for (multipart, out) in tentative {
                match out {
                    Out::Accept => accepted.push(multipart),
                    Out::Reject | Out::Ignore => continue,
                    Out::Cont(dst) => {
                        let mut result = self.0.get(&dst).unwrap().apply_multipart(&multipart);
                        new_tentative.append(&mut result);
                    }
                }
            }

            if new_tentative.is_empty() {
                break;
            }

            tentative = new_tentative;
        }

        accepted
            .into_iter()
            .map(|multipart| multipart.get_num_combinations())
            .sum()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn get_rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Multipart {
    x: RangeInclusive<usize>,
    m: RangeInclusive<usize>,
    a: RangeInclusive<usize>,
    s: RangeInclusive<usize>,
}

impl Multipart {
    fn new() -> Self {
        Self {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }

    fn get_num_combinations(&self) -> usize {
        [&self.x, &self.m, &self.a, &self.s]
            .iter()
            .map(|range| range.end() - range.start() + 1)
            .product()
    }

    fn is_empty(&self) -> bool {
        self.x.is_empty() && self.m.is_empty() && self.a.is_empty() && self.s.is_empty()
    }
}

impl From<&str> for Part {
    fn from(line: &str) -> Part {
        let line = &line[1..line.len() - 1];
        let mut splits = line.split(',');

        let x = splits.next().unwrap();
        let m = splits.next().unwrap();
        let a = splits.next().unwrap();
        let s = splits.next().unwrap();

        Self {
            x: usize::from_str(&x[2..]).unwrap(),
            m: usize::from_str(&m[2..]).unwrap(),
            a: usize::from_str(&a[2..]).unwrap(),
            s: usize::from_str(&s[2..]).unwrap(),
        }
    }
}

impl aoc::Solver for Solver {
    type Input = (Workflows, Vec<Part>);
    type Output1 = usize;
    type Output2 = usize;

    fn parse(input: &str) -> Self::Input {
        let (workflows, parts) = input.split_once("\n\n").unwrap();
        let workflows = Workflows(workflows.lines().map(|line| parse_workflow(line)).collect());
        let parts = parts.lines().map(|line| line.into()).collect();
        (workflows, parts)
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let (workflows, parts) = input;

        parts
            .iter()
            .filter(|part| workflows.accepts(part))
            .map(|part| part.get_rating())
            .sum()
    }

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let (workflows, _) = input;
        workflows.get_num_accepted_ratings()
    }
}

fn main() {
    aoc::run::<Solver>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> <Solver as aoc::Solver>::Input {
        (
            Workflows(
                [
                    (
                        "px".to_string(),
                        Workflow {
                            conds: vec![
                                Cond::Test {
                                    prop: Prop::A,
                                    op: Op::Lt,
                                    crit: 2006,
                                    out: Out::Cont("qkq".to_string()),
                                },
                                Cond::Test {
                                    prop: Prop::M,
                                    op: Op::Gt,
                                    crit: 2090,
                                    out: Out::Accept,
                                },
                                Cond::Always(Out::Cont("rfg".to_string())),
                            ],
                        },
                    ),
                    (
                        "pv".to_string(),
                        Workflow {
                            conds: vec![
                                Cond::Test {
                                    prop: Prop::A,
                                    op: Op::Gt,
                                    crit: 1716,
                                    out: Out::Reject,
                                },
                                Cond::Always(Out::Accept),
                            ],
                        },
                    ),
                    (
                        "lnx".to_string(),
                        Workflow {
                            conds: vec![
                                Cond::Test {
                                    prop: Prop::M,
                                    op: Op::Gt,
                                    crit: 1548,
                                    out: Out::Accept,
                                },
                                Cond::Always(Out::Accept),
                            ],
                        },
                    ),
                    (
                        "rfg".to_string(),
                        Workflow {
                            conds: vec![
                                Cond::Test {
                                    prop: Prop::S,
                                    op: Op::Lt,
                                    crit: 537,
                                    out: Out::Cont("gd".to_string()),
                                },
                                Cond::Test {
                                    prop: Prop::X,
                                    op: Op::Gt,
                                    crit: 2440,
                                    out: Out::Reject,
                                },
                                Cond::Always(Out::Accept),
                            ],
                        },
                    ),
                    (
                        "qs".to_string(),
                        Workflow {
                            conds: vec![
                                Cond::Test {
                                    prop: Prop::S,
                                    op: Op::Gt,
                                    crit: 3448,
                                    out: Out::Accept,
                                },
                                Cond::Always(Out::Cont("lnx".to_string())),
                            ],
                        },
                    ),
                    (
                        "qkq".to_string(),
                        Workflow {
                            conds: vec![
                                Cond::Test {
                                    prop: Prop::X,
                                    op: Op::Lt,
                                    crit: 1416,
                                    out: Out::Accept,
                                },
                                Cond::Always(Out::Cont("crn".to_string())),
                            ],
                        },
                    ),
                    (
                        "crn".to_string(),
                        Workflow {
                            conds: vec![
                                Cond::Test {
                                    prop: Prop::X,
                                    op: Op::Gt,
                                    crit: 2662,
                                    out: Out::Accept,
                                },
                                Cond::Always(Out::Reject),
                            ],
                        },
                    ),
                    (
                        "in".to_string(),
                        Workflow {
                            conds: vec![
                                Cond::Test {
                                    prop: Prop::S,
                                    op: Op::Lt,
                                    crit: 1351,
                                    out: Out::Cont("px".to_string()),
                                },
                                Cond::Always(Out::Cont("qqz".to_string())),
                            ],
                        },
                    ),
                    (
                        "qqz".to_string(),
                        Workflow {
                            conds: vec![
                                Cond::Test {
                                    prop: Prop::S,
                                    op: Op::Gt,
                                    crit: 2770,
                                    out: Out::Cont("qs".to_string()),
                                },
                                Cond::Test {
                                    prop: Prop::M,
                                    op: Op::Lt,
                                    crit: 1801,
                                    out: Out::Cont("hdj".to_string()),
                                },
                                Cond::Always(Out::Reject),
                            ],
                        },
                    ),
                    (
                        "gd".to_string(),
                        Workflow {
                            conds: vec![
                                Cond::Test {
                                    prop: Prop::A,
                                    op: Op::Gt,
                                    crit: 3333,
                                    out: Out::Reject,
                                },
                                Cond::Always(Out::Reject),
                            ],
                        },
                    ),
                    (
                        "hdj".to_string(),
                        Workflow {
                            conds: vec![
                                Cond::Test {
                                    prop: Prop::M,
                                    op: Op::Gt,
                                    crit: 838,
                                    out: Out::Accept,
                                },
                                Cond::Always(Out::Cont("pv".to_string())),
                            ],
                        },
                    ),
                ]
                .into_iter()
                .collect(),
            ),
            vec![
                Part {
                    x: 787,
                    m: 2655,
                    a: 1222,
                    s: 2876,
                },
                Part {
                    x: 1679,
                    m: 44,
                    a: 2067,
                    s: 496,
                },
                Part {
                    x: 2036,
                    m: 264,
                    a: 79,
                    s: 2244,
                },
                Part {
                    x: 2461,
                    m: 1339,
                    a: 466,
                    s: 291,
                },
                Part {
                    x: 2127,
                    m: 1623,
                    a: 2188,
                    s: 1013,
                },
            ],
        )
    }

    #[test]
    fn parsing() {
        let input = r"px{a<2006:qkq,m>2090:A,rfg}
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
{x=2127,m=1623,a=2188,s=1013}";

        assert_eq!(<Solver as aoc::Solver>::parse(input), get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(<Solver as aoc::Solver>::part_1(&get_input()), 19114);
    }

    #[test]
    fn part_2() {
        assert_eq!(
            <Solver as aoc::Solver>::part_2(&get_input()),
            167409079868000
        );
    }
}
