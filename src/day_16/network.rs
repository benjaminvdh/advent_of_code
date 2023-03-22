use std::collections::HashMap;
use std::ops::Index;

#[derive(Clone, Debug, PartialEq)]
pub struct Valve {
    pub flow_rate: usize,
    pub tunnels: HashMap<String, usize>,
}

impl Valve {
    pub fn new<T: AsRef<str>>(flow_rate: usize, tunnels: impl Iterator<Item = T>) -> Self {
        Self {
            flow_rate,
            tunnels: tunnels
                .map(|name| (String::from(name.as_ref()), 1))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Network {
    network: HashMap<String, Valve>,
}

impl Index<&str> for Network {
    type Output = Valve;

    fn index(&self, index: &str) -> &Self::Output {
        &self.network[index]
    }
}

impl Network {
    pub fn new(mut network: HashMap<String, Valve>) -> Self {
        collapse(&mut network);
        remove_empty_valves(&mut network);

        Self { network }
    }
}

fn collapse(network: &mut HashMap<String, Valve>) {
    let clone = network.clone();

    for (name, valve) in network.iter_mut() {
        valve.tunnels = get_transitive_connections(&clone, name.to_owned());
    }
}

fn remove_empty_valves(network: &mut HashMap<String, Valve>) {
    let clone = network.clone();

    for (_, valve) in network.iter_mut() {
        valve.tunnels.retain(|name, _| clone[name].flow_rate > 0);
    }
}

fn get_transitive_connections(
    network: &HashMap<String, Valve>,
    base: String,
) -> HashMap<String, usize> {
    let mut connections = HashMap::new();

    connections.insert(base, 0);

    for i in 1.. {
        let new: Vec<_> = network
            .iter()
            .filter(|(name, value)| {
                !connections.contains_key(*name)
                    && value.tunnels.keys().any(|t| connections.contains_key(t))
            })
            .map(|(name, _)| (name.to_owned(), i))
            .collect();

        if new.is_empty() {
            break;
        }

        for (a, b) in new {
            connections.insert(a, b);
        }
    }

    connections.retain(|_, value| *value > 0);

    connections
}
