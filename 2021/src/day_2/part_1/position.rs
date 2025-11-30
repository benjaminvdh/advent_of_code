use crate::day_2::command::Command;

pub struct Position {
    forward: i32,
    depth: i32,
}

impl Position {
    pub fn new() -> Self {
        Self {
            forward: 0,
            depth: 0,
        }
    }

    pub fn pilot(&mut self, command: &Command) {
        match command {
            Command::Forward(delta) => self.forward += delta,
            Command::Up(delta) => self.depth -= delta,
            Command::Down(delta) => self.depth += delta,
        }
    }

    pub fn get_answer(&self) -> i32 {
        self.forward * self.depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward() {
        let mut position = Position::new();
        let command = Command::Forward(5);

        position.pilot(&command);

        assert_eq!(position.forward, 5);
        assert_eq!(position.depth, 0);
    }

    #[test]
    fn test_down() {
        let mut position = Position::new();
        let command = Command::Down(5);

        position.pilot(&command);

        assert_eq!(position.forward, 0);
        assert_eq!(position.depth, 5);
    }

    #[test]
    fn test_up() {
        let mut position = Position::new();
        let command = Command::Up(3);

        position.pilot(&command);

        assert_eq!(position.forward, 0);
        assert_eq!(position.depth, -3);
    }
}
