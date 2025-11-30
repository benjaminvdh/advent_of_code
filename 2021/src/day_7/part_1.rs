use crate::day_7::Crab;

mod propulsion;

use propulsion::EfficientPropulsion;

pub fn get_least_fuel(crabs: &[Crab]) -> u32 {
    let propulsion = EfficientPropulsion;

    crate::day_7::get_least_fuel(crabs, &propulsion)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let crabs: Vec<_> = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
            .into_iter()
            .map(|pos| Crab::from(pos))
            .collect();

        assert_eq!(get_least_fuel(&crabs), 37);
    }
}
