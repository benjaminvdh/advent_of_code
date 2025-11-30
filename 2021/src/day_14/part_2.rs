use std::collections::HashMap;

use crate::day_14;
use crate::day_14::{Polymer, Rules};

pub fn apply_transformations(input: &[Polymer]) -> usize {
    let (template, rules) = day_14::get_template_and_rules(input);

    get_count(&template, &rules, 40)
}

fn get_count(template: &str, rules: &Rules, num_iterations: usize) -> usize {
    let occurrences = get_num_characters(template, rules, num_iterations);

    if let Some((_, min)) = occurrences.iter().min_by(|(_, v1), (_, v2)| v1.cmp(v2)) {
        if let Some((_, max)) = occurrences.iter().max_by(|(_, v1), (_, v2)| v1.cmp(v2)) {
            return max - min;
        }
    }

    0
}

fn get_num_characters(
    template: &str,
    rules: &Rules,
    num_iterations: usize,
) -> HashMap<char, usize> {
    if num_iterations == 1 {
        get_transformed_occurrences(template, rules)
    } else {
        let rules_halfway = transform_rules_n_times(rules, num_iterations / 2);
        let polymer = transform_polymer(&template, &rules_halfway);

        let polymer = if num_iterations % 2 == 1 {
            transform_polymer(&polymer, rules)
        } else {
            polymer
        };

        get_transformed_occurrences(&polymer, &rules_halfway)
    }
}

fn transform_rules_n_times(rules: &Rules, num_iterations: usize) -> Rules {
    if num_iterations == 1 {
        rules.clone()
    } else {
        let rules_halfway = transform_rules_n_times(rules, num_iterations / 2);
        let rules_halfway_doubled = transform_rules(&rules_halfway, &rules_halfway);

        if num_iterations % 2 == 1 {
            transform_rules(&rules_halfway_doubled, rules)
        } else {
            rules_halfway_doubled
        }
    }
}

fn transform_rules(input: &Rules, rules: &Rules) -> Rules {
    let mut new_rules = HashMap::new();

    for (key, value) in input {
        let transformed = transform_polymer(value, rules);

        new_rules.insert(key.clone(), transformed);
    }

    new_rules
}

fn transform_polymer(polymer: &str, rules: &Rules) -> String {
    let mut result = String::new();

    for i in 0..polymer.chars().count() - 1 {
        let substring = &polymer[i..=i + 1];

        let part = rules.get(substring).unwrap();

        result.push_str(&part[0..part.chars().count() - 1]);
    }

    result.push(polymer.chars().last().unwrap_or_default());

    result
}

fn get_occurrences_per_rule(rules: &Rules) -> HashMap<String, HashMap<char, usize>> {
    let mut map = HashMap::new();

    for (template, expansion) in rules {
        let mut num_occurrences = HashMap::new();
        let expansion = &expansion[0..expansion.chars().count() - 1];

        for c in expansion.chars() {
            let count = num_occurrences.entry(c).or_insert(0);
            *count += 1;
        }

        map.insert(template.clone(), num_occurrences);
    }

    map
}

fn get_transformed_occurrences(polymer: &str, rules: &Rules) -> HashMap<char, usize> {
    let occurrences = get_occurrences_per_rule(&rules);

    let mut map = HashMap::new();

    for i in 0..polymer.chars().count() - 1 {
        let substr = &polymer[i..=i + 1];
        let o = occurrences.get(substr).unwrap();

        for (c, num_occurrences) in o {
            let count = map.entry(*c).or_insert(0);
            *count += num_occurrences;
        }
    }

    *map.entry(polymer.chars().last().unwrap()).or_insert(0) += 1;

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_rules() -> HashMap<String, String> {
        let mut rules = HashMap::new();

        let _ = rules.insert(String::from("CH"), String::from("CBH"));
        let _ = rules.insert(String::from("HH"), String::from("HNH"));
        let _ = rules.insert(String::from("CB"), String::from("CHB"));
        let _ = rules.insert(String::from("NH"), String::from("NCH"));
        let _ = rules.insert(String::from("HB"), String::from("HCB"));
        let _ = rules.insert(String::from("HC"), String::from("HBC"));
        let _ = rules.insert(String::from("HN"), String::from("HCN"));
        let _ = rules.insert(String::from("NN"), String::from("NCN"));
        let _ = rules.insert(String::from("BH"), String::from("BHH"));
        let _ = rules.insert(String::from("NC"), String::from("NBC"));
        let _ = rules.insert(String::from("NB"), String::from("NBB"));
        let _ = rules.insert(String::from("BN"), String::from("BBN"));
        let _ = rules.insert(String::from("BB"), String::from("BNB"));
        let _ = rules.insert(String::from("BC"), String::from("BBC"));
        let _ = rules.insert(String::from("CC"), String::from("CNC"));
        let _ = rules.insert(String::from("CN"), String::from("CCN"));

        rules
    }

    #[test]
    fn test_10_steps() {
        let template = String::from("NNCB");
        assert_eq!(get_count(&template, &get_rules(), 10), 1588);
    }

    fn get_simple_rules() -> HashMap<String, String> {
        let mut rules = HashMap::new();

        let _ = rules.insert(String::from("AA"), String::from("ABA"));
        let _ = rules.insert(String::from("AB"), String::from("ACB"));
        let _ = rules.insert(String::from("AC"), String::from("AAC"));
        let _ = rules.insert(String::from("BA"), String::from("BCA"));
        let _ = rules.insert(String::from("BB"), String::from("BBB"));
        let _ = rules.insert(String::from("BC"), String::from("BAC"));
        let _ = rules.insert(String::from("CA"), String::from("CBA"));
        let _ = rules.insert(String::from("CB"), String::from("CAB"));
        let _ = rules.insert(String::from("CC"), String::from("CCC"));

        rules
    }

    fn get_expected_num_characters(polymer: &str, num_iterations: usize) -> usize {
        (polymer.chars().count() - 1) * 2usize.pow(num_iterations as u32) + 1usize
    }

    #[test]
    fn test_simple_1() {
        test_simple_n(1);
    }

    #[test]
    fn test_simple_2() {
        test_simple_n(2);
    }

    #[test]
    fn test_simple_4() {
        let _reference = String::from("ACBCABAACABCAACAB");

        test_simple_n(4);
    }

    #[test]
    fn test_simple_5() {
        test_simple_n(5);
    }

    #[test]
    fn test_simple_8() {
        let _reference: String = String::from(
            "
        AACABACBACBCABAA
        CBACBACBABAACBAC
        BCAACABCACBCABAA
        CABCAACABCAACABC
        ACBCABAACABCAACA
        BACBABAACBACBACB
        ABAACBACBCAACABC
        AACABACBACBCABAA
        CABCAACABCAACABC
        ACBCABAACABCAACA
        BCAACABCACBCABAA
        CABCAACABCAACABC
        ABAACBACBCAACABC
        AACABACBACBCABAA
        CABCAACABCAACABC
        ACBCABAACABCAACA
        B",
        )
        .split_whitespace()
        .collect();
        test_simple_n(8);
    }

    fn test_simple_n(n: usize) {
        let rules = get_simple_rules();
        let template = String::from("AB");

        let occurrences = get_num_characters(&template, &rules, n);
        assert_eq!(
            occurrences.values().sum::<usize>(),
            get_expected_num_characters(&template, n)
        );
    }

    #[test]
    fn test_simple_20() {
        test_simple_n(20);
    }

    #[test]
    #[ignore]
    fn test_simple_40() {
        test_simple_n(40);
    }
}
