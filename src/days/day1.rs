use itertools::Itertools;

use crate::helper;

pub fn run(input_data: &[(&str, &str)]) {
    let data = parse_data(helper::get_file_data_by_name(input_data, "day1"));

    let answer_1 = count_depth_increases(&data);
    let answer_2 = count_increases_by_windows(&data);

    dbg!(answer_1);
    dbg!(answer_2);
}

fn parse_data(s: String) -> Vec<i64> {
    s.lines()
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect()
}

fn count_depth_increases(nums: &[i64]) -> usize {
    nums.windows(2).filter(|w| w[0] < w[1]).count()
}

fn count_increases_by_windows(nums: &[i64]) -> usize {
    nums.windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::days::day1::{count_depth_increases, count_increases_by_windows};

    #[test]
    fn problem_1_part_1() {
        let measurements = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_depth_increases(&measurements), 7);
    }

    #[test]
    fn problem_1_part_2() {
        let measurements = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increases_by_windows(&measurements), 5);
    }
}
