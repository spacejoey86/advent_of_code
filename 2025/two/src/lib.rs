use std::ops::RangeInclusive;

pub fn part_one(input: &str) -> u64 {
    return id_iterator(input).filter(|id| id_invalid(*id)).sum();
}

pub fn id_iterator(input: &str) -> impl Iterator<Item = u64> {
    input.split(",").flat_map(|range| {
        let (lower, upper) = range.split_once("-").unwrap();
        RangeInclusive::new(lower.parse::<u64>().unwrap(), upper.parse().unwrap())
    })
}

fn id_invalid(id: u64) -> bool {
    let num_digits = id.to_string().len() as u32;
    if num_digits % 2 != 0 {
        return false;
    }
    id % (10_u64.pow(num_digits / 2) + 1) == 0
}

pub fn part_two(input: &str) -> u64 {
    return id_iterator(input).filter(|id| p2_id_invalid(*id)).sum();
}

fn p2_id_invalid(id: u64) -> bool {
    let string_id = id.to_string();
    for i in 1..string_id.len() {
        let prefix = &string_id[..i];
        if is_prefix(prefix, &string_id[i..]) {
            return true;
        }
    }
    return false;
}

fn is_prefix(prefix: &str, suffix: &str) -> bool {
    let mut repeated = prefix.to_string();
    while repeated.len() < suffix.len() {
        repeated.push_str(prefix);
    }
    return repeated == suffix;
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

    #[test]
    fn prefix() {
        assert!(is_prefix("test", "test"))
    }

    #[test]
    fn double_prefix() {
        assert!(is_prefix("oh", "ohoh"))
    }

    #[test]
    fn not_prefix() {
        assert!(!is_prefix("t", "to"))
    }

    #[test]
    fn p2_id() {
        assert!(p2_id_invalid(121212))
    }

    #[test]
    fn part_two_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part_two(&input), 4174379265)
    }

    #[test]
    fn part_two_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(part_two(&input), 47477053982)
    }
}
