use crate::helper;

pub fn run(input_data: &[(&str, &str)]) {
    let data = parse_data(helper::get_file_data_by_name(input_data, "day3"));
}

fn parse_data(s: String) -> anyhow::Result<BingoGame> {
    let mut lines = s.lines();

    let numbers_to_draw = lines
        .next()
        .ok_or(helper::ParseError::Expected("expected nums to draw"))?
        .trim()
        .split(',')
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect::<Vec<u32>>();

    let mut boards: Vec<Board<BoardNumber>> = Vec::new();

    let mut current_board = Board::<BoardNumber>::new(5, 5);
    let mut current_row_count = 5;

    let mut current_board_values: Vec<Vec<usize>> = Vec::new();

    loop {
        let next_line = lines.next();

        // eof reached
        if next_line.is_none() {
            break;
        }

        if let Some("") = next_line {
            current_board = Board::<BoardNumber>::new(5, 5);
            current_row_count = 5;

            continue;
        }



        let row_numbers = next_line
            .ok_or(helper::ParseError::Expected("expected nums to draw"))?
            .trim()
            .split_whitespace()
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .collect::<Vec<usize>>();

        current_board_values.push(row_numbers);

        current_row_count -= 1;
    }

    Ok(BingoGame {
        numbers_to_draw,
        boards,
    })
}

struct BingoGame {
    numbers_to_draw: Vec<u32>,
    boards: Vec<Board<BoardNumber>>,
}

#[derive(Clone, Copy, Default)]
struct BoardNumber {
    value: usize,
    marked: bool,
}
impl BoardNumber {
    pub fn new(value: usize) -> Self {
        Self {
            value,
            marked: false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Board<T> {
    array: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Board<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Copy,
    {
        Self {
            array: [T::default()].repeat(width * height),
            width,
            height,
        }
    }
}

pub trait Gridlike<T> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    /// Get the element at the given point.
    fn get(&self, p: Point) -> &T;

    /// Set all elements of the grid, using a setter function.
    /// The setter function takes a point and returns the value which should be
    /// assigned to the grid at that point.
    fn set_all<F>(&mut self, setter: F)
    where
        F: Send + Sync + Fn(Point) -> T,
        T: Send;
}

impl<T> Gridlike<T> for Board<T> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get(&self, p: Point) -> &T {
        &self.array[p.y * self.width + p.x]
    }

    fn set_all<F>(&mut self, setter: F)
    where
        F: Send + Sync + Fn(Point) -> T,
        T: Send,
    {
        let width = self.width;
        for (i, item) in self.array.iter_mut().enumerate() {
            *item = setter(Point {
                x: i % width,
                y: i / width,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day4::parse_data;

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part_1() {
        let data = parse_data(String::from(INPUT));
    }

    #[test]
    fn part_2() {
        let data = parse_data(String::from(INPUT));
    }
}
