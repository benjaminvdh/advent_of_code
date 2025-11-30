mod position;

use crate::day_2::command::Command;

use position::Position;

pub fn execute_commands(commands: &[Command]) -> i32 {
    let mut position = Position::new();

    for command in commands {
        position.pilot(command);
    }

    position.get_answer()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_input() {
        let commands = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];

        assert_eq!(execute_commands(&commands), 150);
    }

    #[test]
    fn no_input() {
        let commands = vec![];

        assert_eq!(execute_commands(&commands), 0);
    }
}
