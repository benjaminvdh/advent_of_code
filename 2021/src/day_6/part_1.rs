use crate::day_6::LanternFish;

pub const NUM_DAYS: u32 = 80;

pub fn compute_num_lanternfish(mut fish: Vec<LanternFish>) -> u64 {
    for _ in 0..NUM_DAYS {
        let newborn_fish: Vec<_> = fish
            .iter_mut()
            .filter_map(|fish| fish.grow_older())
            .collect();
        fish.extend(newborn_fish.into_iter());
    }

    fish.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_fish() -> Vec<LanternFish> {
        vec![
            LanternFish::from(3),
            LanternFish::from(4),
            LanternFish::from(3),
            LanternFish::from(1),
            LanternFish::from(2),
        ]
    }

    #[test]
    fn test_80_days() {
        assert_eq!(compute_num_lanternfish(get_test_fish()), 5934);
    }
}
