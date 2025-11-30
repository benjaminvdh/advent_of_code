pub mod part_1;
pub mod part_2;

pub mod parsing;

pub use parsing::Parser;

use std::collections::HashMap;

use parsing::Polymer;

type Rules = HashMap<String, String>;

pub fn print_success_message(quantity: usize) {
    println!(
        "The quantity of the most common element minus that of the least common is {}.",
        quantity
    );
}

pub fn print_error_message(error: crate::InputError) {
    eprintln!("Failed to solve the problem: {}.", error);
}

fn get_template_and_rules(input: &[Polymer]) -> (String, Rules) {
    let mut template = String::new();
    let mut rules = Rules::new();

    for input in input {
        match input {
            Polymer::Template(t) => template = t.to_owned(),
            Polymer::Rule(from, to) => {
                let _ = rules.insert(from.to_owned(), to.to_owned());
            }
            _ => (),
        }
    }

    (template, rules)
}
