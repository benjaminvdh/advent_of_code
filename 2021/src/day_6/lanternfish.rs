#[derive(Debug, PartialEq)]
pub struct LanternFish {
    pub timer: u8,
}

impl LanternFish {
    pub fn new() -> Self {
        Self { timer: 8 }
    }

    pub fn grow_older(&mut self) -> Option<Self> {
        match self.timer.checked_sub(1) {
            Some(result) => {
                self.timer = result;
                None
            }
            None => {
                self.timer = 6;
                Some(LanternFish::new())
            }
        }
    }
}

impl From<u8> for LanternFish {
    fn from(age: u8) -> Self {
        Self { timer: age }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grow_older() {
        let mut fish = LanternFish::from(2);

        assert!(fish.grow_older().is_none());
        assert!(fish.grow_older().is_none());
        assert!(fish.grow_older().is_some());
        assert!(fish.grow_older().is_none());
    }
}
