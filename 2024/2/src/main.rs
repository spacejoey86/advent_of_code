use itertools::Itertools;
use std::{cmp::max, fs::read_to_string};

fn main() {
    println!("{}", count_of_safe_reports("input.txt"));
}

fn count_of_safe_reports(path: &str) -> usize {
    let reports: Vec<Vec<i32>> = read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|element| element.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    reports
        .into_iter()
        .filter(|report| is_safe_with_dampening((*report).clone()))
        .count()
}

fn dampened_direction(report: Vec<i32>) -> bool {
    let directions: Vec<Direction> = report
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| Direction::from_samples(a, b))
        .collect();

    if directions.contains(&Direction::Constant) {
        return false;
    }

    let num_increasing = directions
        .clone()
        .into_iter()
        .filter(|dir| *dir == Direction::Increasing)
        .count();
    let num_decreasing = directions
        .clone()
        .into_iter()
        .filter(|dir| *dir == Direction::Decreasing)
        .count();
    let num_constant = directions
        .into_iter()
        .filter(|dir| *dir == Direction::Constant)
        .count();

    num_constant == 0 && num_decreasing < 2 || num_increasing < 2 ||
    num_constant == 1 && (num_decreasing == 0 || num_increasing == 0)
}

fn is_safe(mut report: Vec<i32>) -> bool {
    max_window_difference(report.clone()) <= 3
        && match ReportDirection::from_report(report) {
            ReportDirection::Increasing | ReportDirection::Decreasing => true,
            _ => false,
        }
}

fn is_safe_with_dampening(mut report: Vec<i32>) -> bool {
    max_window_difference(report.clone()) <= 3 &&
        dampened_direction(report)
}

fn max_window_difference(report: Vec<i32>) -> i32 {
    report
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| (a - b).abs())
        .reduce(|total, new| max(total, new))
        .unwrap()
}

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    Increasing,
    Decreasing,
    Constant,
}

impl Direction {
    fn from_samples(first: i32, second: i32) -> Self {
        if first > second {
            Self::Decreasing
        } else if first < second {
            Self::Increasing
        } else {
            Self::Constant
        }
    }

    fn to_report_direction(self) -> ReportDirection {
        match self {
            Direction::Increasing => ReportDirection::Increasing,
            Direction::Decreasing => ReportDirection::Decreasing,
            Direction::Constant => ReportDirection::Constant,
        }
    }
}

#[derive(PartialEq, Debug)]
enum ReportDirection {
    Increasing,
    Decreasing,
    Constant,
    Inconsistent,
}

impl ReportDirection {
    fn from_report(report: Vec<i32>) -> Self {
        let mut current_direction: Option<Direction> = None;
        for (prev_level, level) in report.into_iter().tuple_windows() {
            let new_direction = Direction::from_samples(prev_level, level);
            match &current_direction {
                None => {
                    current_direction = Some(new_direction);
                }
                Some(current_direction) => {
                    if *current_direction == new_direction {
                        continue;
                    } else {
                        return ReportDirection::Inconsistent;
                    }
                }
            }
        }

        match current_direction {
            None => ReportDirection::Constant,
            Some(dir) => dir.to_report_direction(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_increase() {
        assert_eq!(Direction::from_samples(1, 2), Direction::Increasing);
    }

    #[test]
    fn direction_decrease() {
        assert_eq!(Direction::from_samples(5, 3), Direction::Decreasing);
    }

    #[test]
    fn direction_constant() {
        assert_eq!(Direction::from_samples(6, 6), Direction::Constant);
    }

    #[test]
    fn test_decreasing() {
        assert_eq!(
            ReportDirection::from_report(vec![7, 6, 4, 2, 1]),
            ReportDirection::Decreasing
        );
    }

    #[test]
    fn test_increasing() {
        assert_eq!(
            ReportDirection::from_report(vec![1, 2, 3, 4, 5]),
            ReportDirection::Increasing
        );
    }

    #[test]
    fn test_inconsistent() {
        assert_eq!(
            ReportDirection::from_report(vec![1, 6, 3, 4, 5]),
            ReportDirection::Inconsistent
        );
    }

    #[test]
    fn test_constant() {
        assert_eq!(
            ReportDirection::from_report(vec![1, 1, 1]),
            ReportDirection::Constant
        );
    }

    #[test]
    fn test_empty_is_constant() {
        assert_eq!(
            ReportDirection::from_report(vec![]),
            ReportDirection::Constant
        );
    }

    #[test]
    fn large_difference() {
        assert_eq!(max_window_difference(vec![1, 2, 7, 8, 9]), 5)
    }
}
