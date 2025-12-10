use chumsky::{prelude::*, text::whitespace};
use itertools::Itertools;

#[derive(PartialEq, Debug)]
struct Button {
    indicator_indexes: Vec<usize>,
}

#[derive(PartialEq, Debug)]
struct Machine {
    target_indicators: Vec<bool>,
    buttons: Vec<Button>,
    joltage_requirements: Vec<i64>,
}

impl Machine {
    fn new() -> Self {
        Self {
            target_indicators: vec![],
            buttons: vec![],
            joltage_requirements: vec![],
        }
    }

    fn check_combination(&self, combination: &Vec<usize>) -> bool {
        let mut indicators = self.target_indicators.clone();
        for button_index in combination {
            let button = &self.buttons[*button_index];
            for indicator_index in &button.indicator_indexes {
                indicators[*indicator_index] ^= true;
            }
        }

        return !indicators.iter().any(|elem| *elem == true);
    }

    fn min_presses(&self) -> i64 {
        button_combinations(self.buttons.len())
            .find(|button_combination| self.check_combination(button_combination))
            .unwrap()
            .len() as i64
    }
}

fn target_indicator_parser<'a>() -> impl Parser<'a, &'a str, Vec<bool>> {
    let indicator_parser = (just('.').to(false)).or(just('#').to(true));
    just('[')
        .ignore_then(indicator_parser.repeated().collect())
        .then_ignore(just("]"))
}

fn button_parser<'a>() -> impl Parser<'a, &'a str, Button> {
    just('(').ignore_then(
        (text::int(10).map(|i: &str| i.parse::<usize>().unwrap()))
            .separated_by(just(','))
            .collect()
            .map(|vec| Button {
                indicator_indexes: vec,
            })
            .then_ignore(just(')')),
    )
}

fn buttons_parser<'a>() -> impl Parser<'a, &'a str, Vec<Button>> {
    button_parser().separated_by(whitespace()).collect()
}

fn joltage_requirements_parser<'a>() -> impl Parser<'a, &'a str, Vec<i64>> {
    just('{')
        .ignore_then(
            (text::int(10).map(|i: &str| i.parse::<i64>().unwrap()))
                .separated_by(just(','))
                .collect(),
        )
        .then_ignore(just('}'))
}

fn line_parser<'a>() -> impl Parser<'a, &'a str, Machine> {
    target_indicator_parser()
        .then_ignore(whitespace())
        .then(buttons_parser())
        .then_ignore(whitespace())
        .then(joltage_requirements_parser())
        .map(
            |((target_indicators, buttons), joltage_requirements)| Machine {
                target_indicators,
                buttons,
                joltage_requirements,
            },
        )
}

fn parse(input: &str) -> impl Iterator<Item = Machine> {
    input.lines().map(|line| line_parser().parse(line).unwrap())
}

fn button_combinations(num_buttons: usize) -> impl Iterator<Item = Vec<usize>> {
    // no point of doing the same one twice
    // since that undoes things
    // and the order doesn't matter either

    // so we just need combinations?
    // starting with less buttons
    // each iteration should be usizes representing buttons to activate

    // example for 3 buttons:
    // 0
    // 1
    // 2
    // 0, 1
    // 0, 2
    // 1, 2
    // 0, 1, 2

    (1..=num_buttons)
        .flat_map(move |num_to_activate| (0..num_buttons).combinations(num_to_activate))
}

fn part_one(input: &str) -> i64 {
    let machines = parse(input);
    machines.map(|machine| machine.min_presses()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            line_parser()
                .parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")
                .unwrap(),
            Machine {
                target_indicators: vec![false, true, true, false],
                buttons: vec![
                    Button {
                        indicator_indexes: vec![3]
                    },
                    Button {
                        indicator_indexes: vec![1, 3]
                    },
                    Button {
                        indicator_indexes: vec![2]
                    },
                    Button {
                        indicator_indexes: vec![2, 3]
                    },
                    Button {
                        indicator_indexes: vec![0, 2]
                    },
                    Button {
                        indicator_indexes: vec![0, 1]
                    },
                ],
                joltage_requirements: vec![3, 5, 4, 7]
            }
        )
    }

    #[test]
    fn test_button_combinations_n_three() {
        assert_eq!(
            button_combinations(3).collect::<Vec<Vec<usize>>>(),
            vec![
                vec![0],
                vec![1],
                vec![2],
                vec![0, 1],
                vec![0, 2],
                vec![1, 2],
                vec![0, 1, 2],
            ]
        )
    }

    #[test]
    fn test_machine_button_combo() {
        let machine = line_parser()
            .parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")
            .unwrap();
        assert!(machine.check_combination(&vec![0, 1, 2]))
    }

    #[test]
    fn test_machine_min_presses() {
        let machine = line_parser()
            .parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")
            .unwrap();
        assert_eq!(machine.min_presses(), 2)
    }

    #[test]
    fn part_one_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part_one(&input), 7)
    }

    #[test]
    fn part_one_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(part_one(&input), 509)
    }

    // #[test]
    // fn part_two_example_test() {
    //     let input = read_to_string("test_input.txt").unwrap();
    //     assert_eq!(part_two(&input), 33)
    // }

    // #[test]
    // fn part_two_test() {
    //     let input = read_to_string("input.txt").unwrap();
    //     assert_eq!(part_two(&input), 0)
    // }
}
