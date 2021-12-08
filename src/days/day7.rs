use itertools::Itertools;

use crate::helper;

pub fn run(input_data: &[(&str, &str)]) -> anyhow::Result<()> {
    let data = parse_data(helper::get_file_data_by_name(input_data, "day7"));

    let ideal_position = ideal_horizontal_position(data.clone())?;
    let answer_1 = calculate_total_fuel_cost(data.clone(), ideal_position);

    let ideal_position2 = ideal_horizontal_position_pt2(data.clone())?;
    let answer_2 = calculate_total_fuel_cost_pt2(data, ideal_position2);

    dbg!(answer_1);
    dbg!(answer_2);
    Ok(())
}

fn parse_data(s: String) -> Vec<isize> {
    s.lines()
        .next()
        .unwrap()
        .split(',')
        .map(str::parse::<isize>)
        .map(Result::unwrap)
        .collect_vec()
}

fn ideal_horizontal_position(positions: Vec<isize>) -> anyhow::Result<isize> {
    let (min, max) = (
        *positions.iter().min().unwrap(),
        *positions.iter().max().unwrap(),
    );

    let fuel_costs = (min..max)
        .map(|i| (i, calculate_total_fuel_cost(positions.clone(), i)))
        .collect::<Vec<(isize, isize)>>();

    let (ideal_position, _cost) = fuel_costs
        .iter()
        .min_by(|x, y| x.1.cmp(&y.1))
        .ok_or(helper::ParseError::Expected("expected ideal position"))?;

    Ok(*ideal_position)
}

fn determine_fuel_cost(from_position: isize, to_position: isize) -> isize {
    (from_position - to_position).abs()
}

fn calculate_total_fuel_cost(positions: Vec<isize>, target_position: isize) -> isize {
    positions
        .into_iter()
        .map(|p| determine_fuel_cost(p, target_position))
        .sum()
}

fn ideal_horizontal_position_pt2(positions: Vec<isize>) -> anyhow::Result<isize> {
    let (min, max) = (
        *positions.iter().min().unwrap(),
        *positions.iter().max().unwrap(),
    );

    let mut fuel_costs: Vec<(isize, isize)> = vec![];

    for i in min..max {
        let total_cost = calculate_total_fuel_cost_pt2(positions.clone(), i);

        fuel_costs.push((i, total_cost));
    }

    let (ideal_position, _cost) = fuel_costs
        .iter()
        .min_by(|x, y| x.1.cmp(&y.1))
        .ok_or(helper::ParseError::Expected("expected ideal position"))?;

    Ok(*ideal_position)
}

fn calculate_total_fuel_cost_pt2(positions: Vec<isize>, target_position: isize) -> isize {
    positions
        .into_iter()
        .map(|p| determine_fuel_cost_pt2(p, target_position))
        .sum()
}

fn determine_fuel_cost_pt2(from_position: isize, to_position: isize) -> isize {
    let diff = (from_position - to_position).abs();
    (0..diff + 1).into_iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::days::day7::{
        calculate_total_fuel_cost, calculate_total_fuel_cost_pt2, ideal_horizontal_position,
        ideal_horizontal_position_pt2, parse_data,
    };

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part_1() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT));

        let ideal_position = ideal_horizontal_position(data.clone())?;

        assert_eq!(ideal_position.clone(), 2);
        assert_eq!(calculate_total_fuel_cost(data, ideal_position), 37);
        Ok(())
    }

    #[test]
    fn part_2() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT));

        let ideal_position = ideal_horizontal_position_pt2(data.clone())?;

        assert_eq!(ideal_position.clone(), 5);
        assert_eq!(calculate_total_fuel_cost_pt2(data, ideal_position), 168);
        Ok(())
    }
}
