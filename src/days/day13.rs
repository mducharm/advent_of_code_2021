use itertools::Itertools;

use crate::helper;

pub fn run(input_data: &[(&str, &str)]) -> anyhow::Result<()> {
    let mut data = parse_data(helper::get_file_data_by_name(input_data, "day13"));
    let mut data2 = parse_data(helper::get_file_data_by_name(input_data, "day13"));

    data.perform_fold(FoldDirection::X, 655);
    let answer_1 = data.total_visible_dots();

    data2.perform_all_folds();
    let answer_2 = data2.total_visible_dots();

    data2.print();

    dbg!(answer_1);
    dbg!(answer_2);

    Ok(())
}

fn parse_data(input: String) -> Grid<bool> {
    let pairs = input
        .lines()
        .take_while(|s| !s.is_empty())
        .map(|s| {
            s.split(',')
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .take(2)
                .collect_tuple::<(usize, usize)>()
                .unwrap()
        })
        .collect_vec();

    println!("{:?}", pairs);

    let fold_instructions = input
        .lines()
        .rev()
        .take_while(|s| !s.is_empty())
        .collect_vec()
        .iter()
        .rev()
        .map(|s| {
            s.split_whitespace()
                .rev()
                .next()
                .unwrap()
                .split('=')
                .take(2)
                .collect_tuple::<(&str, &str)>()
                .unwrap()
        })
        .map(|f| {
            (
                match f.0 {
                    "x" => FoldDirection::X,
                    "y" => FoldDirection::Y,
                    _ => FoldDirection::Invalid,
                },
                f.1.parse::<usize>().unwrap(),
            )
        })
        .collect_vec();

    let (width, height) = get_width_and_height(pairs.clone());

    let mut grid = Grid::new(width, height, false);

    println!("{:?}", width);
    println!("{:?}", height);
    println!("{:?}", grid.values.len());

    for (x, y) in pairs {
        grid.set(x, y, true);
    }

    for instruction in fold_instructions {
        grid.fold_instructions.push(instruction);
    }

    grid
}

fn get_width_and_height(pairs: Vec<(usize, usize)>) -> (usize, usize) {
    let x_coords = pairs.clone().into_iter().map(|p| p.0).collect_vec();
    let y_coords = pairs.into_iter().map(|p| p.1).collect_vec();

    (
        x_coords.into_iter().max().unwrap(),
        y_coords.into_iter().max().unwrap(),
    )
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum FoldDirection {
    X,
    Y,
    Invalid,
}

#[derive(Debug)]
struct Grid<T> {
    width: usize,
    height: usize,
    values: Vec<T>,
    fold_instructions: Vec<(FoldDirection, usize)>,
}

impl<T: Copy> Grid<T> {
    fn new(width: usize, height: usize, default_value: T) -> Grid<T> {
        Grid {
            width: width + 1,
            height: height + 1,
            values: vec![default_value; (width + 1) * (height + 1)],
            fold_instructions: vec![],
        }
    }

    fn get(&self, x: usize, y: usize) -> T {
        self.values[x + y * self.width]
    }

    fn set(&mut self, x: usize, y: usize, value: T) {
        self.values[x + y * self.width] = value;
    }
}

impl Grid<bool> {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        for row in self.values.chunks(self.width) {
            println!();
            for value in row {
                let c = if *value { "#" } else { "." };
                print!("{}", c);
            }
        }
        println!();
        println!();
        println!("Fold instructions");

        for (d, value) in &self.fold_instructions {
            println!("{:?} = {}", d, value);
        }
    }

    fn total_visible_dots(&self) -> usize {
        self.values.iter().filter(|v| **v).count()
    }

    fn perform_fold(&mut self, direction: FoldDirection, amount: usize) {
        if direction == FoldDirection::Y {
            let new_height = (self.height - 1) / 2;

            let mut coords_to_transpose = vec![];

            for y in amount..self.height {
                for x in 0..self.width {
                    if self.get(x, y) {
                        coords_to_transpose.push((x, y));
                    }
                }
            }

            for (x, y) in coords_to_transpose {
                // let mut new_y = self.height % y;
                // if new_y > 0 { new_y -= 1; };
                self.set(x, (self.height - 1) % y, true);
                self.set(x, y, false);
                println!("fold dir {:?} amount {:?}", direction, amount);
                println!("height {:?}", self.height);
                println!("y {:?}", y);
                println!("{:?}", self.height % y);
                // self.set(x, new_y, true);

            }

            self.height = new_height;
        }

        if direction == FoldDirection::X {
            let new_width = (self.width - 1) / 2;

            let mut coords_to_transpose = vec![];

            for x in amount..self.width {
                for y in 0..self.height {
                    if self.get(x, y) {
                        coords_to_transpose.push((x, y));
                    }
                }
            }

            for (x, y) in coords_to_transpose {
                self.set((self.width - 1) % x, y, true);
                self.set(x, y, false);
            }


            self.width = new_width;
        }
    }

    fn perform_all_folds(&mut self) {
        for (direction, amount) in &self.fold_instructions.clone() {
            self.perform_fold(*direction, *amount);
        }

    }
}

#[cfg(test)]
mod tests {

    use crate::days::day13::FoldDirection;

    use super::parse_data;

    const INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn part_1() -> anyhow::Result<()> {
        let mut data = parse_data(String::from(INPUT));

        println!("{:#?}", &data);

        data.perform_fold(FoldDirection::Y, 7);
        assert_eq!(data.total_visible_dots(), 17);

        data.perform_fold(FoldDirection::X, 5);

        assert_eq!(data.total_visible_dots(), 16);

        Ok(())
    }

    // #[test]
    // fn part_2() -> anyhow::Result<()> {
    //     let data = parse_data(String::from(INPUT));

    //     todo!();

    //     Ok(())
    // }
}
