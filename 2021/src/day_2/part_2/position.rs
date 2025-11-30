use crate::day_2::command::Command;

pub struct Position {
    forward: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    pub fn new() -> Self {
        Self {
            forward: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn pilot(&mut self, command: &Command) {
        match command {
            Command::Forward(delta) => self.move_forward(*delta),
            Command::Up(delta) => self.aim -= delta,
            Command::Down(delta) => self.aim += delta,
        }
    }

    fn move_forward(&mut self, delta: i32) {
        self.forward += delta;
        self.depth += delta * self.aim;
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
        assert_eq!(position.aim, 0);
    }

    #[test]
    fn test_down() {
        let mut position = Position::new();
        let command = Command::Down(5);

        position.pilot(&command);

        assert_eq!(position.forward, 0);
        assert_eq!(position.depth, 0);
        assert_eq!(position.aim, 5);
    }

    #[test]
    fn test_up() {
        let mut position = Position::new();
        let command = Command::Up(3);

        position.pilot(&command);

        assert_eq!(position.forward, 0);
        assert_eq!(position.depth, 0);
        assert_eq!(position.aim, -3);
    }

    #[test]
    fn test_down_and_forward() {
        let mut position = Position::new();

        let commands = &[Command::Down(5), Command::Forward(8)];

        for command in commands {
            position.pilot(command);
        }

        assert_eq!(position.forward, 8);
        assert_eq!(position.depth, 40);
        assert_eq!(position.aim, 5);
    }
}
