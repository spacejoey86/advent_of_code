pub fn part_one(input: &str) -> i64 {
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    lines
        .last()
        .unwrap()
        .into_iter()
        .enumerate()
        .map(|(i, operation)| {
            let elements: Vec<i64> = lines
                .iter()
                .rev()
                .skip(1)
                .map(|l| l.into_iter().skip(i).next().unwrap().parse().unwrap())
                .collect();
            if *operation == "+" {
                elements.into_iter().sum::<i64>()
            } else {
                elements.into_iter().product()
            }
        })
        .sum()
}

fn parse_p2_lines(input: &str) -> (Vec<&str>, Vec<&str>) {
    let lines: Vec<&str> = input.lines().collect();
    let operations = lines.iter().last().unwrap().split_whitespace().collect();

    return (operations, lines.into_iter().rev().skip(1).rev().collect());
}

fn parse_to_cells(lines: Vec<&str>) -> Vec<Vec<String>> {
    let mut cells: Vec<Vec<String>> = lines.iter().map(|_| vec!["".into()]).collect();
    for index in 0..lines[0].len() {
        if lines.iter().all(|line| line.as_bytes()[index] == b' ') {
            // move to next column
            for row in cells.iter_mut() {
                row.push("".into());
            }
        } else {
            for (row_index, row) in cells.iter_mut().enumerate() {
                row.last_mut()
                    .unwrap()
                    .push(lines[row_index].as_bytes()[index] as char);
            }
        }
    }
    return cells;
}

fn transpose(operations: Vec<&str>, cells: Vec<Vec<String>>) -> Vec<(&str, Vec<String>)> {
    operations
        .into_iter()
        .enumerate()
        .map(|(index, operation)| {
            let cells_vertical = cells
                .iter()
                .map(|row| (row.iter().skip(index).next().unwrap().to_owned()))
                .collect();
            (operation, cells_vertical)
        })
        .collect()
}

fn part_two(input: &str) -> i64 {
    let (operations, lines) = parse_p2_lines(input);
    let cells = parse_to_cells(lines);
    let columns = transpose(operations, cells);
    columns
        .into_iter()
        .map(|(operation, cells)| {
            let elements = (0..cells[0].len()).map(|i| {
                let string = String::from_utf8(
                    cells
                        .iter()
                        .map(|c| c.as_bytes().iter().skip(i).next().unwrap().clone())
                        .collect(),
                )
                .unwrap();
                string.trim().parse::<i64>().unwrap()
            });
            if operation == "+" {
                elements.into_iter().sum::<i64>()
            } else {
                elements.into_iter().product()
            }
        })
        .sum()
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

    #[test]
    fn part_two_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part_two(&input), 3263827)
    }

    #[test]
    fn part_two_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(part_two(&input), 10603075273949)
    }
}
