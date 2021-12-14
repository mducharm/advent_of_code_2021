use std::collections::HashMap;

use itertools::Itertools;

use crate::helper;

pub fn run(input_data: &[(&str, &str)]) -> anyhow::Result<()> {
    let mut data = parse_data(helper::get_file_data_by_name(input_data, "day14"));

    data.apply_rules_x_times(10);
    let answer_1 = data.most_common_minus_least_common();

    data.apply_rules_x_times(30);
    let answer_2 = data.most_common_minus_least_common();

    dbg!(answer_1);
    dbg!(answer_2);

    Ok(())
}

fn parse_data(input: String) -> PolymerizationProcess {
    let mut s = input.lines();

    let template: Vec<char> = s.next().unwrap().chars().collect_vec();

    s.next();

    let mut insertion_rules = vec![];

    for line in s {
        let mut instruction = line.split_whitespace();

        let pattern: (char, char) = instruction
            .next()
            .unwrap()
            .chars()
            .take(2)
            .collect_tuple()
            .unwrap();

        instruction.next();

        let element_to_insert: char = instruction.next().unwrap().chars().next().unwrap();

        insertion_rules.push(InsertionRule {
            pattern,
            element_to_insert,
        })
    }

    PolymerizationProcess {
        template,
        insertion_rules,
    }
}

struct PolymerizationProcess {
    template: Vec<char>,
    insertion_rules: Vec<InsertionRule>,
}

#[derive(Clone, Copy)]
struct InsertionRule {
    pattern: (char, char),
    element_to_insert: char,
}

impl PolymerizationProcess {
    fn apply_insertion_rules(&mut self) {
        let mut windows = self
            .template
            .clone()
            .into_iter()
            .tuple_windows();

        let mut new_template: Vec<char> = vec![];

        let first_pair: (char, char) = windows.next().unwrap();

        new_template.push(first_pair.0);

        for rule in self.insertion_rules.clone() {
            if first_pair == rule.pattern {
                new_template.push(rule.element_to_insert);
            }
        }
        new_template.push(first_pair.1);

        for pair in windows {
            let (_a, b) = pair;

            for rule in self.insertion_rules.clone() {
                if pair == rule.pattern {
                    new_template.push(rule.element_to_insert);
                }
            }
                
            new_template.push(b);
        }
        self.template = new_template;
    }

    fn apply_rules_x_times(&mut self, x: usize) {
        for _i in 0..x {
            self.apply_insertion_rules();
        }
    }

    fn most_common_minus_least_common(&self) -> usize {
        self.most_common_element() - self.least_common_element()
    }

    fn most_common_element(&self) -> usize {
        *self.get_element_counts()
            .iter()
            .map(|(c, v)| v)
            .max()
            .unwrap()
    }

    fn least_common_element(&self) -> usize {
        *self.get_element_counts()
            .iter()
            .map(|(c, v)| v)
            .min()
            .unwrap()
    }

    fn get_element_counts(&self) -> HashMap<char, usize> {
        self.template
            .clone()
            .into_iter()
            .fold(HashMap::new(), |mut acc, el| {
                *acc.entry(el).or_insert(0) += 1;
                acc
            })
    }
}

#[cfg(test)]
mod tests {

    use super::parse_data;

    const INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn part_1() -> anyhow::Result<()> {
        let mut data = parse_data(String::from(INPUT));

        data.apply_rules_x_times(1);
        assert_eq!(String::from_iter(data.template.clone()), "NCNBCHB");

        data.apply_rules_x_times(1);
        assert_eq!(String::from_iter(data.template.clone()), "NBCCNBBBCBHCB");

        data.apply_rules_x_times(1);
        assert_eq!(String::from_iter(data.template.clone()), "NBBBCNCCNBBNBNBBCHBHHBCHB");

        data.apply_rules_x_times(1);
        assert_eq!(String::from_iter(data.template.clone()), "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");

        data.apply_rules_x_times(6);

        assert_eq!(data.least_common_element(), 161);
        assert_eq!(data.most_common_element(), 1749);
        assert_eq!(data.most_common_minus_least_common(), 1588);

        Ok(())
    }

    #[test]
    fn part_2() -> anyhow::Result<()> {
        let mut data = parse_data(String::from(INPUT));

        data.apply_rules_x_times(40);

        assert_eq!(data.least_common_element(), 3849876073);
        assert_eq!(data.most_common_element(), 2192039569602);
        assert_eq!(data.most_common_minus_least_common(), 2188189693529);

        Ok(())
    }
}
