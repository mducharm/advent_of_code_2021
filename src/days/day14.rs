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

    let pair_count: HashMap<(char, char), usize> =
        s.next()
            .unwrap()
            .chars()
            .tuple_windows()
            .fold(HashMap::new(), |mut acc, pair| {
                *acc.entry(pair).or_insert(0) += 1;
                acc
            });

    s.next();

    let mut rules = HashMap::new();

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

        let (fst, snd) = pattern;

        rules
            .entry(pattern)
            .or_insert(((fst, element_to_insert), (element_to_insert, snd)));

    }

    println!("{:?}", pair_count);
    println!("{:?}", rules);

    PolymerizationProcess {
        pair_count,
        insertion_rules: InsertionRules(rules),
    }
}

struct PolymerizationProcess {
    pair_count: HashMap<(char, char), usize>,
    insertion_rules: InsertionRules,
}

struct InsertionRules(HashMap<(char, char), ((char, char), (char, char))>);


impl PolymerizationProcess {
    fn apply_insertion_rules(&mut self) {
        let mut new_pair_count = HashMap::new();

        for (pair, count) in self.pair_count.clone() {
            if let Some((fst, snd)) = self.insertion_rules.0.get(&pair) {
                *new_pair_count.entry(*fst).or_insert(0) += count;
                *new_pair_count.entry(*snd).or_insert(0) += count;
            }
        }

        self.pair_count = new_pair_count;
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
        *self
            .get_element_counts()
            .iter()
            .map(|(_c, v)| v)
            .max()
            .unwrap()
    }

    fn least_common_element(&self) -> usize {
        *self
            .get_element_counts()
            .iter()
            .map(|(_c, v)| v)
            .min()
            .unwrap()
    }

    fn get_element_counts(&self) -> HashMap<char, usize> {

        let mut count = HashMap::new();

        for ((fst, snd), amount) in self.pair_count.clone(){
            *count.entry(fst).or_insert(0) += amount;
            *count.entry(snd).or_insert(0) += amount;
        }

        for (c, amount) in count.clone() {
            *count.entry(c).or_insert(0) = (amount + 1) / 2;
        }

        count
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

        data.apply_rules_x_times(10);

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
