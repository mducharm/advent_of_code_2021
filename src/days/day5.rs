use itertools::Itertools;

use crate::helper;

pub fn run(input_data: &[(&str, &str)]) -> anyhow::Result<()> {
    let data = parse_data(helper::get_file_data_by_name(input_data, "day5"))?;
    let data2 = parse_data(helper::get_file_data_by_name(input_data, "day5"))?;

    let answer_1 = count_intersecting_lines(data);
    let answer_2 = count_all_intersecting_lines(data2);

    dbg!(answer_1);
    dbg!(answer_2);
    Ok(())
}

fn parse_data(input: String) -> anyhow::Result<Vec<Segment>> {
    Ok(input
        .lines()
        .map(|s| parse_line_into_segment(s))
        .map(Result::unwrap)
        .collect_vec())
}

fn parse_line_into_segment(input: &str) -> anyhow::Result<Segment> {
    let mut point_str = input.split(" -> ");

    let str_to_point = |s: &str| -> anyhow::Result<Point> {
        let mut p = s.split(',');
        let x = p.next().map(str::parse::<u16>).unwrap()?;
        let y = p.next().map(str::parse::<u16>).unwrap()?;
        Ok(Point { x, y })
    };

    let first = &point_str.next().map(str_to_point).unwrap()?;
    let second = &point_str.next().map(str_to_point).unwrap()?;

    Ok(Segment(*first, *second))
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn as_tuple(&self) -> (u16, u16) {
        (self.x, self.y)
    }
    fn plot_point_on_grid(&self, grid: &mut Vec<Vec<u16>>) {
        let (x, y) = self.as_tuple();

        grid[y as usize][x as usize] += 1;
    }
}

#[derive(Debug)]
struct Segment(Point, Point);

impl Segment {
    fn not_diagonal(&self) -> bool {
        let (x1, y1) = self.0.as_tuple();
        let (x2, y2) = self.1.as_tuple();

        x1 == x2 || y1 == y2
    }

    fn get_all_points(&self) -> Vec<Point> {
        let (x1, y1) = self.0.as_tuple();
        let (x2, y2) = self.1.as_tuple();

        if self.not_diagonal() {
            if x1 == x2 {
                if y2 > y1 {
                    (y1..y2 + 1).map(|y| Point { x: x1, y }).collect_vec()
                } else {
                    (y2..y1 + 1).map(|y| Point { x: x1, y }).collect_vec()
                }
            } else {
                if x2 > x1 {
                    (x1..x2 + 1).map(|x| Point { x, y: y1 }).collect_vec()
                } else {
                    (x2..x1 + 1).map(|x| Point { x, y: y1 }).collect_vec()
                }
            }
        } else {
            let x_range = if x2 > x1 {
                (x1..x2 + 1).collect_vec()
            } else {
                (x2..x1 + 1).rev().collect_vec()
            };
            let y_range = if y2 > y1 {
                (y1..y2 + 1).collect_vec()
            } else {
                (y2..y1 + 1).rev().collect_vec()
            };

            x_range
                .into_iter()
                .zip(y_range.into_iter())
                .map(|(x, y)| Point { x, y })
                .collect_vec()

            // x_range.into_iter().enumerate(|(i, x)| Point {
            //     x,
            //     y: y_range.interleave(other)
            // })

            // (x1..x2 + 1).map(|x| Point { x, y: y1 }).collect_vec()
        }
    }
}

// part 1

fn count_intersecting_lines(segments: Vec<Segment>) -> usize {
    let mut grid = vec![vec![0u16; 1000]; 1000];

    let non_diagonal_segments = segments.iter().filter(|s| s.not_diagonal());

    for segment in non_diagonal_segments {
        let related_points = segment.get_all_points();
        for point in related_points {
            point.plot_point_on_grid(&mut grid);
        }
    }

    let mut overlapping_points: usize = 0;

    for row in grid.iter() {
        for value in row.iter() {
            if value > &1 {
                overlapping_points += 1;
            }
        }
    }

    overlapping_points
}

fn count_all_intersecting_lines(segments: Vec<Segment>) -> usize {
    // let mut grid = vec![vec![0u16; 1000]; 1000];
    let mut grid = vec![vec![0u16; 1000]; 1000];

    for segment in segments {
        let related_points = segment.get_all_points();
        // for r in &related_points {
        //     println!("{:?}", r);
        // }
        for point in related_points {
            point.plot_point_on_grid(&mut grid);
        }
        // for g in &grid {
        //     println!("{:?}", g);
        // }
    }

    // for g in &grid {
    //     println!("{:?}", g);
    // }

    let mut overlapping_points: usize = 0;

    for row in grid.iter() {
        for value in row.iter() {
            if value > &1 {
                overlapping_points += 1;
            }
        }
    }

    overlapping_points
}

#[cfg(test)]
mod tests {

    use crate::days::day5::{count_all_intersecting_lines, count_intersecting_lines};

    use super::parse_data;

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part_1() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT))?;

        assert_eq!(count_intersecting_lines(data), 5);

        Ok(())
    }

    #[test]
    fn part_2() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT))?;

        assert_eq!(count_all_intersecting_lines(data), 12);

        Ok(())
    }
}
