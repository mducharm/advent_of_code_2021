use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::helper;

pub fn run(input_data: &[(&str, &str)]) -> anyhow::Result<()> {
    let data = parse_data(helper::get_file_data_by_name(input_data, "day8"));
    let data2 = parse_data(helper::get_file_data_by_name(input_data, "day8"));

    let answer_1 = count_unique_output_digits(data);
    let answer_2 = sum_output_digits(data2);

    dbg!(answer_1);
    dbg!(answer_2);
    Ok(())
}

fn parse_data(s: String) -> Vec<Entry> {
    s.lines().map(|l| parse_line_into_entry(l)).collect_vec()
}

fn parse_line_into_entry(s: &str) -> Entry {
    let (patterns, output_str) = s.split(" | ").next_tuple().unwrap();

    let signal_patterns = patterns.split_whitespace().map(String::from).collect();
    let output = output_str.split_whitespace().map(String::from).collect();

    Entry {
        signal_patterns,
        output,
    }
}

#[allow(dead_code)]
struct Entry {
    signal_patterns: Vec<String>,
    output: Vec<String>,
}

// part 1
fn count_unique_output_digits(entries: Vec<Entry>) -> usize {
    let mut sum: usize = 0;
    for Entry {
        signal_patterns: _,
        output,
    } in entries
    {
        sum += output
            .into_iter()
            .map(|s| match s.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            })
            .sum::<usize>();
    }
    sum
}

// part 2
fn sum_output_digits(entries: Vec<Entry>) -> u128 {
    convert_entries(entries).into_iter().map(get_digit).sum()
}

#[derive(Clone)]
struct Entry2 {
    signal_patterns: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
}

fn convert_entries(entries: Vec<Entry>) -> Vec<Entry2> {
    entries
        .into_iter()
        .map(|e| Entry2 {
            signal_patterns: e
                .signal_patterns
                .into_iter()
                .map(|s| s.chars().collect::<HashSet<char>>())
                .into_iter()
                .collect_vec(),
            output: e
                .output
                .into_iter()
                .map(|s| s.chars().collect::<HashSet<char>>())
                .into_iter()
                .collect_vec(),
        })
        .collect_vec()
}

fn get_digit(entry: Entry2) -> u128 {
    let digit_map = get_digit_map(entry.clone());

    let digits_as_tuples: Vec<(usize, HashSet<char>)> = digit_map.into_iter().collect_vec();
    let mut digits: Vec<String> = vec![];

    for d in entry.output {
        let (num, pattern) = digits_as_tuples
            .iter()
            .find(|t| t.1 == d)
            .ok_or(helper::ParseError::Expected(
                "expected to match existing pattern",
            ))
            .unwrap();
        digits.push(num.to_string());
    }

    digits.join("").parse::<u128>().unwrap()
}

fn get_digit_map(entry: Entry2) -> HashMap<usize, HashSet<char>> {
    // let mut signal_patterns= vec![];

    let mut by_len: HashMap<usize, Vec<HashSet<char>>> = HashMap::new();
    let mut by_digit: HashMap<usize, HashSet<char>> = HashMap::new();

    for pattern in entry.signal_patterns {
        let len = pattern.len();

        let map_entry = by_len.entry(len).or_insert(Vec::new());

        map_entry.push(pattern);
    }

    // unique digits
    by_digit.insert(1, by_len.get(&2).unwrap().iter().next().unwrap().to_owned());
    by_digit.insert(1, by_len.get(&2).unwrap().iter().next().unwrap().to_owned());
    by_digit.insert(4, by_len.get(&4).unwrap().iter().next().unwrap().to_owned());
    by_digit.insert(7, by_len.get(&3).unwrap().iter().next().unwrap().to_owned());
    by_digit.insert(8, by_len.get(&7).unwrap().iter().next().unwrap().to_owned());

    let len_of_6 = by_len.get(&6).unwrap().clone();
    let len_of_5 = by_len.get(&5).unwrap().clone();

    by_digit.insert(
        6,
        len_of_6
            .iter()
            .find(|h| {
                h.difference(by_digit.get(&1).unwrap())
                    .collect::<HashSet<&char>>()
                    .len()
                    == 5
            })
            .unwrap()
            .to_owned(),
    );

    by_digit.insert(
        3,
        len_of_5
            .iter()
            .find(|h| {
                h.difference(by_digit.get(&1).unwrap())
                    .collect::<HashSet<&char>>()
                    .len()
                    == 3
            })
            .unwrap()
            .to_owned(),
    );

    by_digit.insert(
        9,
        len_of_6
            .iter()
            .find(|h| {
                by_digit
                    .get(&4)
                    .unwrap()
                    .difference(h)
                    .collect::<HashSet<&char>>()
                    .is_empty()
            })
            .unwrap()
            .to_owned(),
    );

    by_digit.insert(
        0,
        len_of_6
            .iter()
            .find(|h: &&HashSet<char>| {
                *h.to_owned() != (by_digit.get(&6).unwrap().to_owned())
                    && *h.to_owned() != (by_digit.get(&9).unwrap().to_owned())
            })
            .unwrap()
            .to_owned(),
    );
    by_digit.insert(
        2,
        len_of_5
            .iter()
            .find(|h| {
                h.difference(by_digit.get(&1).unwrap())
                    .collect::<HashSet<&char>>()
                    .len()
                    == 4
                    && h.difference(by_digit.get(&6).unwrap())
                        .collect::<HashSet<&char>>()
                        .len()
                        == 1
            })
            .unwrap()
            .to_owned(),
    );

    by_digit.insert(
        5,
        len_of_5
            .iter()
            .find(|h| {
                h.difference(by_digit.get(&1).unwrap())
                    .collect::<HashSet<&char>>()
                    .len()
                    == 4
                    && h.difference(by_digit.get(&6).unwrap())
                        .collect::<HashSet<&char>>()
                        .is_empty()
            })
            .unwrap()
            .to_owned(),
    );

    by_digit
}

#[cfg(test)]
mod tests {
    use crate::days::day8::{count_unique_output_digits, sum_output_digits};

    use super::parse_data;

    const INPUT: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    const INPUT_2: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn part_1() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT));
        let data2 = parse_data(String::from(INPUT_2));

        assert_eq!(count_unique_output_digits(data), 0);
        assert_eq!(count_unique_output_digits(data2), 26);

        Ok(())
    }

    #[test]
    fn part_2() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT));
        let data2 = parse_data(String::from(INPUT_2));

        assert_eq!(sum_output_digits(data), 5353);
        assert_eq!(sum_output_digits(data2), 61229);
        Ok(())
    }
}
