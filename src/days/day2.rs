use crate::helper;

pub fn run(input_data: &[(&str, &str)]) {
    let data = parse_data(helper::get_file_data_by_name(input_data, "day2"));

    let answer_1 = calculate_position(&data);
    let answer_2 = calculate_position_with_aim(data);

    dbg!(answer_1.horizontal * answer_1.depth);
    dbg!(answer_2.horizontal * answer_2.depth);
}

fn parse_data(s: String) -> Vec<Instruction> {
    s.lines()
        .map(parse_line_into_instruction)
        .map(Result::unwrap)
        .collect()
}

fn parse_line_into_instruction(s: &str) -> anyhow::Result<Instruction> {
    let mut tokens = s.split(' ');

    let command = tokens.next();
    let amount = str::parse::<i64>(tokens.next().unwrap()).unwrap();

    let instruction = match command.unwrap() {
        "up" => Instruction::Up(amount),
        "down" => Instruction::Down(amount),
        "forward" => Instruction::Forward(amount),
        _ => Instruction::Noop,
    };

    Ok(instruction)
}

fn calculate_position(instructions: &[Instruction]) -> Position {
    let mut position = Position::origin();

    for instruction in instructions {
        let (h, d) = calcuate_increase(instruction);
        position.horizontal += h;
        position.depth += d;
    }
    position
}
fn calcuate_increase(i: &Instruction) -> (i64, i64) {
    match i {
        Instruction::Up(x) => (0, -*x),
        Instruction::Down(x) => (0, *x),
        Instruction::Forward(x) => (*x, 0),
        _ => (0, 0),
    }
}

fn calculate_position_with_aim(instructions: Vec<Instruction>) -> Position {
    let mut position = Position::origin();
    let mut aim = 0;

    for instruction in instructions {
        if let Instruction::Up(x) = instruction {
            aim -= x;
        }

        if let Instruction::Down(x) = instruction {
            aim += x;
        }

        if let Instruction::Forward(x) = instruction {
            position.horizontal += x;
            position.depth += aim * x;
        }
    }

    Position {
        horizontal: position.horizontal,
        depth: position.depth,
    }
}

#[derive(Debug)]
struct Position {
    horizontal: i64,
    depth: i64,
}

impl Position {
    fn origin() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.horizontal == other.horizontal && self.depth == other.depth
    }
}

enum Instruction {
    Forward(i64),
    Up(i64),
    Down(i64),
    Noop,
}

#[cfg(test)]
mod tests {
    use crate::days::day2::{calculate_position, calculate_position_with_aim, Position};

    use super::parse_data;

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn part_1() {
        let instructions = parse_data(String::from(INPUT));

        assert_eq!(
            calculate_position(&instructions),
            Position {
                horizontal: 15,
                depth: 10
            }
        );
    }

    #[test]
    fn part_2() {
        let instructions = parse_data(String::from(INPUT));

        assert_eq!(
            calculate_position_with_aim(instructions),
            Position {
                horizontal: 15,
                depth: 60,
            }
        );
    }
}
