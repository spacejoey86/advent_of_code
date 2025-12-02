pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn part_one(input: &str) -> u64 {
    let mut sum = 0;

    for range in input.split(",") {
        println!("{}", range);
        let (lower, upper) = range.split_once("-").unwrap();
        println!("{} - {}", lower, upper);
        for id in lower.parse::<u64>().unwrap()..=upper.parse().unwrap() {
            if id_invalid(id) {
                sum += id;
            }
        }
    }

    return sum;
}

fn id_invalid(id: u64) -> bool {
    let num_digits = id.to_string().len() as u32;
    if num_digits % 2 != 0 {
        return false;
    }
    id % (10_u64.pow(num_digits / 2) + 1) == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn id_test() {
        assert_eq!(id_invalid(123123), true);
    }

    #[test]
    fn part_one_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part_one(&input), 1227775554)
    }

    #[test]
    fn part_one_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(part_one(&input), 37314786486)
    }
}
