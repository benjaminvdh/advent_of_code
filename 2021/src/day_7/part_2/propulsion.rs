use crate::day_7::propulsion;

pub struct InefficientPropulsion;

impl propulsion::Propulsion for InefficientPropulsion {
    fn get_fuel_for_position(&self, cur_pos: u32, dest_pos: u32) -> u32 {
        let diff = crate::abs_diff(cur_pos, dest_pos);

        (diff + 1) * diff / 2
    }
}
