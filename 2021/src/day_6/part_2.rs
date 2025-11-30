use crate::day_6::LanternFish;

pub const NUM_DAYS: u32 = 256;

pub fn compute_num_lanternfish(fish: Vec<LanternFish>) -> u64 {
    let mut fish = create_fish_tracker(&fish);

    for _ in 0..NUM_DAYS {
        fish.rotate_left(1);

        fish[6] += fish[8];
    }

    fish.iter().sum::<u64>() as u64
}

fn create_fish_tracker(fish: &[LanternFish]) -> [u64; 9] {
    let mut fish_tracker = [0u64; 9];

    for fish in fish {
        fish_tracker[fish.timer as usize] += 1;
    }

    fish_tracker
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
    fn test_256_days() {
        assert_eq!(compute_num_lanternfish(get_test_fish()), 26984457539);
    }
}
