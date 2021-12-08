use itertools::Itertools;

use crate::helper;

pub fn run(input_data: &[(&str, &str)]) -> anyhow::Result<()> {
    let data = parse_data(helper::get_file_data_by_name(input_data, "day6"));

    let answer_1 = simulate_x_days_of_growth(data.clone(), 80);
    let answer_2 = simulate_x_days_of_growth(data, 256);

    dbg!(answer_1);
    dbg!(answer_2);

    Ok(())
}

fn parse_data(s: String) -> Vec<usize> {
    s.lines()
        .next()
        .unwrap()
        .split(',')
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect_vec()
}

fn simulate_fish_growth(fish: [u128; 9]) -> [u128; 9] {
    let mut counts = fish;

    let fish_ready_to_spawn = counts[0];

    for i in 1..9 {
        counts[i - 1] = counts[i];
    }

    counts[6] += fish_ready_to_spawn;
    counts[8] = fish_ready_to_spawn;
    counts
}

fn simulate_x_days_of_growth(initial_fish: Vec<usize>, days: usize) -> u128 {
    let mut current_fish_count: [u128; 9] = [0; 9];

    for f in initial_fish.into_iter() {
        current_fish_count[f] += 1;
    }

    for _day in 0..days {
        current_fish_count = simulate_fish_growth(current_fish_count);
    }
    
    current_fish_count.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::days::day6::simulate_x_days_of_growth;

    use super::parse_data;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn part_1() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT));

        assert_eq!(simulate_x_days_of_growth(data.clone(), 18), 26);
        assert_eq!(simulate_x_days_of_growth(data, 80), 5934);

        Ok(())
    }

    #[test]
    fn part_2() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT));

        assert_eq!(simulate_x_days_of_growth(data.clone(), 18), 26);
        assert_eq!(simulate_x_days_of_growth(data.clone(), 80), 5934);
        assert_eq!(simulate_x_days_of_growth(data, 256), 26984457539);

        Ok(())
    }
}
