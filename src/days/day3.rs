use crate::helper;

pub fn run(input_data: &[(&str, &str)]) {
    let data = parse_data(helper::get_file_data_by_name(input_data, "day3"));

    let answer_1 = data.get_power_consumption();

    dbg!(answer_1);
}

fn parse_data(s: String) -> DiagnosticReport {
    let values = s
        .lines()
        .map(|s| usize::from_str_radix(s, 2))
        .map(Result::unwrap)
        .collect::<Vec<usize>>();

    let number_of_bits = s.lines().next().unwrap().len();

    DiagnosticReport {
        values,
        number_of_bits,
    }
}

struct DiagnosticReport {
    values: Vec<usize>,
    number_of_bits: usize,
}

impl DiagnosticReport {
    #[allow(unused_variables)]
    fn get_gamma_rate(&self) -> usize {
        let mut bits: Vec<usize> = Vec::new();

        for i in 0..self.number_of_bits {
            dbg!(i);

            let total = &self.values.len();
            let zero_count = &self
                .values
                .to_owned()
                .into_iter()
                .filter(|x| (x >> i) & 1 == 0) // 
                .count();
            let one_count = total - zero_count;

            let bit = if zero_count > &one_count { 0 } else { 1 };
            bits.insert(0, bit);
        }

        let mut gamma_rate = 0;

        for bit in bits {
            gamma_rate <<= 1;
            gamma_rate |= bit;
        }

        gamma_rate
    }

    fn get_epsilon_rate(&self) -> usize {
        let mask = (1 << self.number_of_bits) - 1;
        let inverse = !self.get_gamma_rate();

        inverse & mask
    }

    fn get_power_consumption(&self) -> usize {
        self.get_gamma_rate() * self.get_epsilon_rate()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day3::parse_data;

    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part_1() {
        let data = parse_data(String::from(INPUT));

        assert_eq!(data.get_gamma_rate(), 22);
        assert_eq!(data.get_epsilon_rate(), 9);
        assert_eq!(data.get_power_consumption(), 198);
    }

    #[test]
    fn part_2() {
        // todo!()
    }
}
