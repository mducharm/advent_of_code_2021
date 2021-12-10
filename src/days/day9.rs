use itertools::Itertools;

use crate::helper;

pub fn run(input_data: &[(&str, &str)]) -> anyhow::Result<()> {
    let data = parse_data(helper::get_file_data_by_name(input_data, "day9"));

    let answer_1 = sum_of_risk_levels(data.clone());
    let answer_2 = get_product_of_largest_basins(data);

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

// part 1

#[allow(unused_assignments)]
fn sum_of_risk_levels(height_map: Vec<Vec<u8>>) -> u128 {
    get_all_low_points(&height_map)
        .iter()
        .map(|x| x.value)
        .map(|x| x as u128 + 1)
        .sum()
}

// part 2

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    y: usize,
    x: usize,
    value: u8,
}

#[derive(Debug)]
struct Basin(Vec<Point>);

impl Basin {
    pub fn len(self) -> u128 {
        self.0.len() as u128
    }
}

fn get_all_low_points(height_map: &Vec<Vec<u8>>) -> Vec<Point> {
    let mut points = vec![];
    for (y, row) in height_map.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let adjacent_nums = get_adjacent_points(x, y, height_map)
                .into_iter()
                .map(|a| a.value)
                .collect_vec();

            if adjacent_nums.into_iter().all(|z| z > *value) {
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

fn get_adjacent_points(x: usize, y: usize, height_map: &[Vec<u8>]) -> Vec<Point> {
    let mut adjacent_nums = vec![];
    if y > 0 {
        if let Some(r) = &height_map.get(y - 1) {
            if let Some(n) = r.get(x) {
                adjacent_nums.push(Point {
                    y: y - 1,
                    x,
                    value: *n,
                });
            }
        }
    }
    if x > 0 {
        if let Some(r) = &height_map.get(y) {
            if let Some(n) = r.get(x - 1) {
                adjacent_nums.push(Point {
                    y,
                    x: x - 1,
                    value: *n,
                });
            }
        }
    }
    if let Some(r) = &height_map.get(y) {
        if let Some(n) = r.get(x + 1) {
            adjacent_nums.push(Point {
                y,
                x: x + 1,
                value: *n,
            });
        }
    }
    if let Some(r) = &height_map.get(y + 1) {
        if let Some(n) = r.get(x) {
            adjacent_nums.push(Point {
                y: y + 1,
                x,
                value: *n,
            });
        }
    }
    adjacent_nums
}

fn get_basins_from_low_points(points: Vec<Point>, height_map: &Vec<Vec<u8>>) -> Vec<Basin> {
    let mut basins = vec![];
    for point in points {
        let mut basin_points = vec![point];

        find_basin_points(point, &mut basin_points, height_map);

        basins.push(Basin(basin_points));
    }
    basins
}

fn find_basin_points(point: Point, basin_points: &mut Vec<Point>, height_map: &Vec<Vec<u8>>) {
    let adj_points = get_adjacent_points(point.x, point.y, height_map);

    for p in adj_points {
        let point_already_in_basin = basin_points.iter().any(|bp| *bp == p);

        if !point_already_in_basin && p.value < 9 {
            basin_points.push(p);
            find_basin_points(p, basin_points, height_map);
        }
    }
}

fn get_three_largest_basin_lens(basins: Vec<Basin>) -> Vec<u128> {
    basins
        .into_iter()
        .map(Basin::len)
        .sorted()
        .rev()
        .take(3)
        .collect_vec()
}

fn get_product_of_largest_basins(height_map: Vec<Vec<u8>>) -> u128 {
    let low_points = get_all_low_points(&height_map);
    let basins = get_basins_from_low_points(low_points, &height_map);

    get_three_largest_basin_lens(basins).into_iter().product()
}

#[cfg(test)]
mod tests {

    use crate::days::day9::{get_product_of_largest_basins, sum_of_risk_levels};

    use super::parse_data;

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn part_1() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT));

        assert_eq!(sum_of_risk_levels(data), 15);
        Ok(())
    }

    #[test]
    fn part_2() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT));

        assert_eq!(get_product_of_largest_basins(data), 1134);

        Ok(())
    }
}
