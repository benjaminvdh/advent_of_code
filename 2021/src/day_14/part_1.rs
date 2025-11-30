use std::collections::HashMap;

use crate::day_14;
use crate::day_14::parsing::Polymer;
use crate::day_14::Rules;

pub fn apply_transformations(input: &[Polymer]) -> usize {
    let (template, rules) = day_14::get_template_and_rules(input);

    let polymer = transform_n_times(template, &rules, 10);

    get_count(&polymer)
}

fn transform_n_times(polymer: String, rules: &Rules, num_iterations: usize) -> String {
    (0..num_iterations).fold(polymer, |polymer, _| transform(&polymer, rules))
}

fn get_count(polymer: &str) -> usize {
    let mut counts: HashMap<char, usize> = HashMap::new();

    for c in polymer.chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }

    let min = *counts.values().min().unwrap_or(&0);
    let max = *counts.values().max().unwrap_or(&0);

    max - min
}

fn transform(polymer: &str, rules: &Rules) -> String {
    let mut result = String::new();

    for i in 0..polymer.chars().count() - 1 {
        let substring = &polymer[i..=i + 1];

        result += rules.get(substring).unwrap();
    }

    result.push(polymer.chars().last().unwrap());

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_transformation() {
        let template = String::from("NNCB");

        let mut rules = HashMap::new();
        let _ = rules.insert(String::from("NN"), String::from("NC"));
        let _ = rules.insert(String::from("NC"), String::from("NB"));
        let _ = rules.insert(String::from("CB"), String::from("CH"));

        let transformed = transform(&template, &rules);

        assert_eq!(transformed, String::from("NCNBCHB"));
    }

    fn get_rules() -> Rules {
        let mut rules = HashMap::new();

        let _ = rules.insert(String::from("CH"), String::from("CB"));
        let _ = rules.insert(String::from("HH"), String::from("HN"));
        let _ = rules.insert(String::from("CB"), String::from("CH"));
        let _ = rules.insert(String::from("NH"), String::from("NC"));
        let _ = rules.insert(String::from("HB"), String::from("HC"));
        let _ = rules.insert(String::from("HC"), String::from("HB"));
        let _ = rules.insert(String::from("HN"), String::from("HC"));
        let _ = rules.insert(String::from("NN"), String::from("NC"));
        let _ = rules.insert(String::from("BH"), String::from("BH"));
        let _ = rules.insert(String::from("NC"), String::from("NB"));
        let _ = rules.insert(String::from("NB"), String::from("NB"));
        let _ = rules.insert(String::from("BN"), String::from("BB"));
        let _ = rules.insert(String::from("BB"), String::from("BN"));
        let _ = rules.insert(String::from("BC"), String::from("BB"));
        let _ = rules.insert(String::from("CC"), String::from("CN"));
        let _ = rules.insert(String::from("CN"), String::from("CC"));

        rules
    }

    #[test]
    fn four_transformations() {
        let template = String::from("NNCB");

        let result = transform_n_times(template, &get_rules(), 4);

        assert_eq!(
            result,
            String::from("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB")
        );
    }

    #[test]
    fn ten_transformations() {
        let template = String::from("NNCB");

        let result = transform_n_times(template, &get_rules(), 10);

        assert_eq!(get_count(&result), 1588);
    }
}
