use std::collections::{HashMap, HashSet};

// row first, then column
fn parse(input: &str) -> HashSet<(i32, i32)> {
    let mut grid = HashSet::new();
    for (row_index, row) in input.lines().enumerate() {
        for (column_index, cell) in row.bytes().enumerate() {
            match cell {
                b'@' => {
                    grid.insert((row_index as i32, column_index as i32));
                }
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }

    return grid;
}

fn adjacent(coord: &(i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    return vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .into_iter()
    .map(move |(x, y)| (x + coord.0, y + coord.1));
}

pub fn part_one(input: &str) -> usize {
    let grid = parse(input);
    grid.iter()
        .filter(|coord| adjacent(coord).filter(|c| grid.contains(c)).count() < 4)
        .count()
}

pub fn part_two(input: &str) -> usize {
    let mut grid = parse(input);
    let mut count = 0;
    loop {
        let removable: Vec<(i32, i32)> = grid.iter().map(|item| item.clone())
            .filter(|coord| adjacent(coord).filter(|c| grid.contains(c)).count() < 4).collect();
        if removable.len() == 0 {
            break
        }
        for coord in removable {
            count += 1;
            grid.remove(&coord);
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_parse() {
        let result = parse(".@.\n@..\n");
        assert!(result.contains(&(1, 0)));
        assert!(result.contains(&(0, 1)))
    }

    #[test]
    fn test_adjacent() {
        let adjacent = adjacent(&(0, 0));
        assert!(adjacent.eq(vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1)
        ]))
    }

    #[test]
    fn part_one_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part_one(&input), 13)
    }

    #[test]
    fn part_one_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(part_one(&input), 1433)
    }

    #[test]
    fn part_two_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part_two(&input), 43)
    }

    #[test]
    fn part_two_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(part_two(&input), 8616)
    }
}
