mod network;

use std::collections::HashSet;

use crate::{ParseError, SolveError};

use network::{Network, Valve};

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Network;
    type Output = usize;
    const DAY: u8 = 16;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        Ok(Network::new(
            input
                .lines()
                .map(|line| parse_line(line))
                .collect::<Result<_, _>>()?,
        ))
    }

    fn part_1(network: Self::Input) -> Result<Self::Output, SolveError> {
        let mut max = 0;

        for (a_name, a_dist) in network["AA"].tunnels.iter() {
            let mut opened_valves: HashSet<&str> = HashSet::new();
            opened_valves.insert(a_name);

            max = max.max(get_max_pressure(
                &network,
                a_name,
                *a_dist,
                "AA",
                1000,
                30,
                opened_valves,
                0,
                &mut max,
            ));
        }

        Ok(max)
    }

    fn part_2(network: Self::Input) -> Result<Self::Output, SolveError> {
        let mut max = 0;

        for (a_name, a_dist) in network["AA"].tunnels.iter() {
            for (b_name, b_dist) in network["AA"].tunnels.iter() {
                if a_name > b_name {
                    let mut opened_valves: HashSet<&str> = HashSet::new();
                    opened_valves.insert(a_name);
                    opened_valves.insert(b_name);

                    max = max.max(get_max_pressure(
                        &network,
                        a_name,
                        *a_dist,
                        b_name,
                        *b_dist,
                        26,
                        opened_valves,
                        0,
                        &mut max,
                    ));
                }
            }
        }

        Ok(max)
    }
}

fn get_max_pressure<'a>(
    network: &'a Network,
    dest_a: &'a str,
    distance_a: usize,
    dest_b: &'a str,
    distance_b: usize,
    time_left: usize,
    opened_valves: HashSet<&'a str>,
    flow_so_far: usize,
    max: &mut usize,
) -> usize {
    if distance_a >= time_left && distance_b >= time_left {
        *max = (*max).max(flow_so_far);
        return flow_so_far;
    }

    let closest;
    let furthest;
    let distance_closest;
    let distance_furthest;

    if distance_a <= distance_b {
        closest = dest_a;
        furthest = dest_b;
        distance_closest = distance_a;
        distance_furthest = distance_b;
    } else {
        closest = dest_b;
        furthest = dest_a;
        distance_closest = distance_b;
        distance_furthest = distance_a;
    }

    let closest_flow = (time_left - distance_closest - 1) * network[closest].flow_rate;
    let furthest_flow = network[closest]
        .tunnels
        .get(furthest)
        .map(|d| time_left.saturating_sub(distance_closest + d + 2))
        .unwrap_or(0)
        .max(time_left.saturating_sub(distance_furthest + 1))
        * network[furthest].flow_rate;

    if *max
        > flow_so_far
            + closest_flow
            + furthest_flow
            + network[closest]
                .tunnels
                .iter()
                .filter(|(name, _)| !opened_valves.contains(name.as_str()))
                .map(|(name, dist)| {
                    time_left.saturating_sub(distance_closest + *dist + 2).max(
                        time_left
                            .saturating_sub(distance_closest + network[furthest].tunnels[name] + 2),
                    ) * network[name].flow_rate
                })
                .sum::<usize>()
    {
        0
    } else {
        let next_flow = network[closest]
            .tunnels
            .iter()
            .filter(|(dest_next, distance)| {
                **distance + 1 < time_left - distance_closest
                    && !opened_valves.contains(dest_next.as_str())
            })
            .map(|(dest_next, distance)| {
                let dest_a_next: &str;
                let dest_b_next: &str;
                let distance_a_next;
                let distance_b_next;

                if closest == dest_a {
                    dest_a_next = dest_next;
                    dest_b_next = dest_b;
                    distance_a_next = *distance + 1;
                    distance_b_next = distance_b - distance_closest;
                } else {
                    dest_a_next = dest_a;
                    dest_b_next = dest_next;
                    distance_a_next = distance_a - distance_closest;
                    distance_b_next = *distance + 1;
                }

                let mut opened_valves = opened_valves.clone();
                assert!(opened_valves.insert(dest_next));

                let flow = get_max_pressure(
                    network,
                    dest_a_next,
                    distance_a_next,
                    dest_b_next,
                    distance_b_next,
                    time_left - distance_closest,
                    opened_valves,
                    flow_so_far + closest_flow,
                    max,
                );
                flow
            })
            .max();

        let next_flow = next_flow.unwrap_or_else(|| {
            let distance_a_next;
            let distance_b_next;

            if closest == dest_a {
                distance_a_next = 1000;
                distance_b_next = distance_b - distance_closest;
            } else {
                distance_a_next = distance_a - distance_closest;
                distance_b_next = 1000;
            }

            get_max_pressure(
                network,
                dest_a,
                distance_a_next,
                dest_b,
                distance_b_next,
                time_left - distance_closest,
                opened_valves,
                flow_so_far + closest_flow,
                max,
            )
        });

        next_flow
    }
}

fn parse_line(line: &str) -> Result<(String, Valve), ParseError> {
    let line = line.trim_start_matches("Valve ");
    let (name, line) = line.split_once(' ').ok_or(ParseError::Invalid)?;
    let line = line.trim_start_matches("has flow rate=");
    let (rate, line) = line.split_once(';').ok_or(ParseError::Invalid)?;
    let line = line.trim_start_matches(" tunnels lead to valves ");
    let line = line.trim_start_matches(" tunnel leads to valve ");
    let tunnels = line.split(", ");

    Ok((name.to_owned(), Valve::new(rate.parse()?, tunnels)))
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    use std::collections::HashMap;

    use super::{Network, Valve};

    fn get_input() -> Network {
        let mut network = HashMap::new();

        network.insert("AA".into(), Valve::new(0, ["DD", "II", "BB"].iter()));
        network.insert("BB".into(), Valve::new(13, ["CC", "AA"].iter()));
        network.insert("CC".into(), Valve::new(2, ["DD", "BB"].iter()));
        network.insert("DD".into(), Valve::new(20, ["CC", "AA", "EE"].iter()));
        network.insert("EE".into(), Valve::new(3, ["FF", "DD"].iter()));
        network.insert("FF".into(), Valve::new(0, ["EE", "GG"].iter()));
        network.insert("GG".into(), Valve::new(0, ["FF", "HH"].iter()));
        network.insert("HH".into(), Valve::new(22, ["GG"].iter()));
        network.insert("II".into(), Valve::new(0, ["AA", "JJ"].iter()));
        network.insert("JJ".into(), Valve::new(21, ["II"].iter()));

        Network::new(network)
    }

    #[test]
    fn parsing() {
        let input = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

        assert_eq!(
            super::Solver::parse(String::from(input)).unwrap(),
            get_input()
        );
    }

    #[test]
    fn part_1() {
        let input = get_input();

        assert_eq!(super::Solver::part_1(input).unwrap(), 1651);
    }

    #[test]
    fn part_2() {
        let input = get_input();

        assert_eq!(super::Solver::part_2(input).unwrap(), 1707);
    }
}
