pub trait Propulsion {
    fn get_fuel_for_position(&self, cur_pos: u32, dest_pos: u32) -> u32;
}
