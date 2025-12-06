enum Operation{
    Multiply,
    Add,
}

pub fn part_one(input: &str) -> i64{
    let lines: Vec<Vec<&str>> = input.lines().map(|line|{
        line.split_whitespace().collect()
    }).collect();
    lines.last().unwrap().into_iter().enumerate().map(|(i, operation)| {
        let elements: Vec<i64> = lines.iter().rev().skip(1).map(|l| l.into_iter().skip(i).next().unwrap().parse().unwrap()).collect();
        if *operation == "+" {
            elements.into_iter().sum::<i64>()
        } else {
            elements.into_iter().product()
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn part_one_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part_one(&input), 4277556)
    }

    #[test]
    fn part_one_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(part_one(&input), 6757749566978)
    }
}
