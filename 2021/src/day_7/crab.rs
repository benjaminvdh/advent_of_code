use crate::day_7::Propulsion;

#[derive(Debug, PartialEq)]
pub struct Crab {
    pub pos: u32,
}

impl Crab {
    pub fn get_fuel_for_position<P: Propulsion>(&self, propulsion: &P, pos: u32) -> u32 {
        propulsion.get_fuel_for_position(self.pos, pos)
    }
}

impl From<u32> for Crab {
    fn from(pos: u32) -> Self {
        Self { pos }
    }
}
