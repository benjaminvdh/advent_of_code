use std::collections::HashMap;

use super::{Conf, Module, PulseType};

pub(crate) fn parse(input: &str) -> Conf {
    let mut modules: HashMap<_, _> = input.lines().map(|line| parse_line(line)).collect();

    let missing: Vec<_> = modules
        .values()
        .flat_map(|module| module.get_outputs())
        .filter(|output| !modules.contains_key(output.as_str()))
        .map(|output| output.to_string())
        .collect();

    for missing in missing {
        modules.insert(missing.to_string(), Module::Sink { dummy: vec![] });
    }

    let mut inputs_for_modules = get_inputs_for_modules(&modules);

    for (name, module) in modules.iter_mut() {
        if let Module::Conj { inputs, .. } = module {
            let new_inputs = inputs_for_modules.remove(name).unwrap();
            let new_inputs = new_inputs.into_iter().map(|name| (name, PulseType::Low));
            inputs.extend(new_inputs);
        }
    }

    Conf { modules }
}

fn parse_line(line: &str) -> (String, Module) {
    let (name, outputs) = line.split_once(" -> ").unwrap();

    let first_char = name.chars().next().unwrap();
    let is_flipflop = first_char == '%';
    let is_conj = first_char == '&';

    let name = if is_flipflop || is_conj {
        &name[1..]
    } else {
        name
    }
    .to_string();

    let outputs = outputs
        .split(", ")
        .map(|output| output.to_string())
        .collect();

    let module = if is_flipflop {
        Module::FlipFlop {
            name: name.clone(),
            is_on: false,
            outputs,
        }
    } else if is_conj {
        Module::Conj {
            name: name.clone(),
            inputs: HashMap::new(),
            outputs,
        }
    } else {
        Module::Broadcaster {
            name: name.clone(),
            outputs,
        }
    };

    (name, module)
}

fn get_inputs_for_modules(modules: &HashMap<String, Module>) -> HashMap<String, Vec<String>> {
    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();

    for (name, module) in modules.iter() {
        for output in module.get_outputs() {
            inputs
                .entry(output.to_string())
                .or_insert(vec![])
                .push(name.to_string());
        }
    }

    inputs
}
