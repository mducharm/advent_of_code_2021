use std::os::windows::process;

use itertools::Itertools;

use crate::helper;

pub fn run(input_data: &[(&str, &str)]) -> anyhow::Result<()> {
    let data = parse_data(helper::get_file_data_by_name(input_data, "day11"));

    let answer_1 = num_of_flashes_after_x_steps(data.clone(), 100);
    let answer_2 = get_step_when_all_flash(data);

    dbg!(answer_1);
    dbg!(answer_2);

    Ok(())
}

fn parse_data(input: String) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|s: &str| {
            s.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect_vec()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    y: usize,
    x: usize,
    value: u8,
}


fn get_flashed_points(grid: &Vec<Vec<u8>>) -> Vec<Point> {
    let mut points = vec![];
    for (y, row) in grid.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if value > &9 {
                points.push(Point {
                    y,
                    x,
                    value: *value,
                });
            }
        }
    }
    points
}

fn get_surrounding_points(x: usize, y: usize, grid: &[Vec<u8>]) -> Vec<Point> {
    let mut adjacent_nums = vec![];

    if x > 0 && y > 0 {
        if let Some(r) = &grid.get(y - 1) {
            if let Some(n) = r.get(x - 1) {
                adjacent_nums.push(Point {
                    y: y - 1,
                    x: x - 1,
                    value: *n,
                });
            }
        }
    }
    if y > 0 {
        if let Some(r) = &grid.get(y - 1) {
            if let Some(n) = r.get(x) {
                adjacent_nums.push(Point {
                    y: y - 1,
                    x,
                    value: *n,
                });
            }
        }

        if let Some(r) = &grid.get(y - 1) {
            if let Some(n) = r.get(x + 1) {
                adjacent_nums.push(Point {
                    y: y - 1,
                    x: x + 1,
                    value: *n,
                });
            }
        }
    }
    if x > 0 {
        if let Some(r) = &grid.get(y) {
            if let Some(n) = r.get(x - 1) {
                adjacent_nums.push(Point {
                    y,
                    x: x - 1,
                    value: *n,
                });
            }
        }

        if let Some(r) = &grid.get(y + 1) {
            if let Some(n) = r.get(x - 1) {
                adjacent_nums.push(Point {
                    y: y + 1,
                    x: x - 1,
                    value: *n,
                });
            }
        }
    }

    if let Some(r) = &grid.get(y) {
        if let Some(n) = r.get(x + 1) {
            adjacent_nums.push(Point {
                y,
                x: x + 1,
                value: *n,
            });
        }
    }

    if let Some(r) = &grid.get(y + 1) {
        if let Some(n) = r.get(x) {
            adjacent_nums.push(Point {
                y: y + 1,
                x,
                value: *n,
            });
        }
    }

    if let Some(r) = &grid.get(y + 1) {
        if let Some(n) = r.get(x + 1) {
            adjacent_nums.push(Point {
                y: y + 1,
                x: x + 1,
                value: *n,
            });
        }
    }
    adjacent_nums
}

fn increment_all_by_one(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    grid.into_iter()
        .map(|row| row.into_iter().map(|v| v + 1).collect_vec())
        .collect_vec()
}

fn increment_points_by_one(grid: Vec<Vec<u8>>, points: Vec<Point>) -> Vec<Vec<u8>> {
    let mut new_grid = grid;
    for point in points {
        let Point { x, y, value: _ } = point;
        new_grid[y][x] += 1;
    }
    new_grid
}

fn reset_flash_points(grid: Vec<Vec<u8>>) -> (Vec<Vec<u8>>, usize) {
    let num_of_flashed_points = grid
        .clone()
        .into_iter()
        .map(|row| row.into_iter().filter(|v| v > &9).count())
        .sum();

    let grid_with_updated_flash_points = grid
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|v| if v > 9 { 0 } else { v })
                .collect_vec()
        })
        .collect_vec();

    (grid_with_updated_flash_points, num_of_flashed_points)
}

fn iterate_next_step(grid: Vec<Vec<u8>>) -> (Vec<Vec<u8>>, usize) {
    let grid_plus_one = increment_all_by_one(grid);

    // process all flash points

    let mut points_already_flashed = get_flashed_points(&grid_plus_one);

    let mut g = grid_plus_one;

    let mut flashed_points = get_flashed_points(&g);

    while !flashed_points.is_empty() {
        for point in flashed_points {
            // increment neighbors by 1
            let Point { x, y, value: _ } = point;
            let neighbors = get_surrounding_points(x, y, &g);

            g = increment_points_by_one(g, neighbors);

            // add to list of already flashed
            points_already_flashed.push(point);
        }

        // filter flashed_points based on already flashed
        flashed_points = get_flashed_points(&g)
            .into_iter()
            .filter(|p| -> bool {
                !points_already_flashed
                    .clone()
                    .into_iter()
                    .any(|paf| paf.x == p.x && paf.y == p.y)
            })
            .collect_vec();
    }

    // update flash points
    reset_flash_points(g)
}

fn num_of_flashes_after_x_steps(initial_grid: Vec<Vec<u8>>, steps: usize) -> usize {
    let mut grid = initial_grid;
    let mut num_of_flashes: usize = 0;

    for _i in 0..steps {
        let (new_grid, new_num_of_flashes) = iterate_next_step(grid);

        grid = new_grid;
        num_of_flashes += new_num_of_flashes;
    }

    num_of_flashes
}

// part 2

fn get_step_when_all_flash(initial_grid: Vec<Vec<u8>>) -> usize {
    let mut grid = initial_grid;
    let mut previous_num_of_flashes: usize = 0;

    let total_octopus_count = grid.clone().into_iter().map(|v| v.into_iter().len()).sum();

    let mut step = 0;

    while previous_num_of_flashes != total_octopus_count {
        let (new_grid, new_num_of_flashes) = iterate_next_step(grid);
        step += 1;

        grid = new_grid;
        previous_num_of_flashes = new_num_of_flashes;
    }

    step
}

#[cfg(test)]
mod tests {

    use crate::days::day11::get_step_when_all_flash;

    use super::{num_of_flashes_after_x_steps, parse_data};

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

//     const INPUT2: &str = "11111
// 19991
// 19191
// 19991
// 11111";

    #[test]
    fn part_1() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT));
        // let data2 = parse_data(String::from(INPUT2));

        let flashes1 = num_of_flashes_after_x_steps(data.clone(), 2);
        let flashes2 = num_of_flashes_after_x_steps(data, 100);

        assert_eq!(flashes1, 35);
        assert_eq!(flashes2, 1656);

        Ok(())
    }

    #[test]
    fn part_2() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT));

        let step = get_step_when_all_flash(data);

        assert_eq!(step, 195);

        Ok(())
    }
}
