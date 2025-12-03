pub fn bank_max_joltage(bank: &str) -> u32 {
    // first digit will be the highest battery in the bank
    // and just take the first index of it, will always be best
    let int_bank: Vec<u32> = bank
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect();
    let first_digit = int_bank[..int_bank.len() - 1].iter().max().unwrap();
    let first_digit_index = int_bank
        .iter()
        .position(|item| item == first_digit)
        .unwrap();
    let second_digit = int_bank[first_digit_index + 1..].iter().max().unwrap();
    return *first_digit * 10 + second_digit;
}

pub fn bank_max_joltage_n(bank: &str, number_of_batteries: usize) -> u64 {
    let int_bank: Vec<u32> = bank
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect();
    return bank_max_joltage_n_inner(&int_bank, number_of_batteries, 0);
}

fn bank_max_joltage_n_inner(
    bank: &[u32],
    number_of_batteries: usize,
    prev_digit_total: u64,
) -> u64 {
    if number_of_batteries == 0 {
        return prev_digit_total;
    } else {
        let first_digit = bank[..bank.len() - (number_of_batteries - 1)]
            .iter()
            .max()
            .unwrap();
        let first_digit_index = bank.iter().position(|item| item == first_digit).unwrap();
        return bank_max_joltage_n_inner(
            &bank[first_digit_index + 1..],
            number_of_batteries - 1,
            10 * prev_digit_total + (*first_digit as u64),
        );
    }
}

pub fn bank_iterator(input: &str) -> impl Iterator<Item = &str> {
    return input.lines();
}

pub fn part_one(input: &str) -> u32 {
    return bank_iterator(input)
        .map(|bank| bank_max_joltage(bank))
        .sum();
}

pub fn part_two(input: &str) -> u64 {
    return bank_iterator(input)
        .map(|bank| bank_max_joltage_n(bank, 12))
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn bank_joltage() {
        assert_eq!(bank_max_joltage("192"), 92);
    }

    #[test]
    fn bank_joltage_last_highest() {
        assert_eq!(bank_max_joltage("123"), 23)
    }

    #[test]
    fn part_one_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part_one(&input), 357)
    }

    #[test]
    fn part_one_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(part_one(&input), 17435)
    }

    #[test]
    fn n_joltage_two() {
        assert_eq!(bank_max_joltage_n("192", 2), 92)
    }

    #[test]
    fn n_joltage_last_highest() {
        assert_eq!(bank_max_joltage_n("123", 2), 23)
    }

    #[test]
    fn n_joltage_all_batteries() {
        assert_eq!(bank_max_joltage_n("123", 3), 123)
    }

    #[test]
    fn part_two_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part_two(&input), 3121910778619)
    }

    #[test]
    fn part_two_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(part_two(&input), 172886048065379)
    }
}
