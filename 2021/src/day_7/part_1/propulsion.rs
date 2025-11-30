use crate::day_7::propulsion;

pub struct EfficientPropulsion;

impl propulsion::Propulsion for EfficientPropulsion {
    fn get_fuel_for_position(&self, cur_pos: u32, dest_pos: u32) -> u32 {
        crate::abs_diff(cur_pos, dest_pos)
    }
}
