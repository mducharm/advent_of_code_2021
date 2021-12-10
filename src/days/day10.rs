use itertools::Itertools;

use crate::helper;

pub fn run(input_data: &[(&str, &str)]) -> anyhow::Result<()> {
    let data = parse_data(helper::get_file_data_by_name(input_data, "day10"));
    let data2 = parse_data(helper::get_file_data_by_name(input_data, "day10"));

    let answer_1 = total_syntax_error_score(data);
    let answer_2 = get_middle_score(data2);

    dbg!(answer_1);
    dbg!(answer_2);

    Ok(())
}

fn parse_data(input: String) -> Vec<Vec<Token>> {
    input
        .lines()
        .map(|s: &str| s.chars().map(Token::new).collect_vec())
        .collect_vec()
}

#[derive(Debug)]
struct Token(char);

impl Token {
    fn new(c: char) -> Self {
        Self(c)
    }

    fn is_closed(&self) -> bool {
        matches!(self.0, ')' | ']' | '}' | '>')
    }
    fn valid_chunk(&self, t: &Token) -> bool {
        matches!(
            (self.0, t.0),
            ('{', '}') | ('[', ']') | ('(', ')') | ('<', '>')
        )
    }

    fn get_closing_token(&self) -> Self {
        match self.0 {
            '(' => Self(')'),
            '[' => Self(']'),
            '{' => Self('}'),
            '<' => Self('>'),
            _ => Self('*'),
        }
    }

    fn get_illegal_char_score(&self) -> u32 {
        match self.0 {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }
    }

    fn get_char_score(&self) -> u64 {
        match self.0 {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        }
    }
}

fn process_line(line: Vec<Token>) -> u32 {
    let mut stack: Vec<Token> = vec![];
    for token in line {
        if token.is_closed() {
            if let Some(previous_token) = stack.pop() {
                if !previous_token.valid_chunk(&token) {
                    return token.get_illegal_char_score();
                }
            }
        } else {
            stack.push(token);
        }
    }
    0
}

fn contains_syntax_error(line: &Vec<Token>) -> bool {
    let mut stack: Vec<&Token> = vec![];
    for token in line {
        if token.is_closed() {
            if let Some(previous_token) = stack.pop() {
                if !previous_token.valid_chunk(token) {
                    return true;
                }
            }
        } else {
            stack.push(token);
        }
    }
    false
}

fn total_syntax_error_score(lines: Vec<Vec<Token>>) -> u32 {
    lines.into_iter().map(process_line).sum()
}

fn get_incomplete_line_score(line: &Vec<Token>) -> u64 {
    let mut stack: Vec<&Token> = vec![];

    for token in line {
        if token.is_closed() {
            stack.pop();
        } else {
            stack.push(token);
        }
    }

    println!("{:?}", stack);

    let mut score: u64 = 0;

    while let Some(t) = stack.pop() {
        score *= 5;
        score += t.get_closing_token().get_char_score();
    }
    score
}

fn get_middle_score(lines: Vec<Vec<Token>>) -> u64 {
    let incomplete_lines = lines
        .iter()
        .filter(|l| !contains_syntax_error(*l))
        .map(|line| get_incomplete_line_score(line))
        .sorted()
        .collect_vec();

    *incomplete_lines.get(incomplete_lines.len() / 2).unwrap()
}

#[cfg(test)]
mod tests {

    use crate::days::day10::{get_middle_score, total_syntax_error_score};

    use super::parse_data;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part_1() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT));

        assert_eq!(total_syntax_error_score(data), 26397);
        Ok(())
    }

    #[test]
    fn part_2() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT));

        assert_eq!(get_middle_score(data), 288957);

        Ok(())
    }
}
