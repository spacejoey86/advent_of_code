use std::{cmp::min, collections::HashSet};

use chumsky::{prelude::*, text::whitespace};
use itertools::Itertools;
use rayon::prelude::*;

#[derive(PartialEq, Debug)]
struct Button {
    indicator_indexes: HashSet<usize>,
}

#[derive(PartialEq, Debug)]
struct MachineP2 {
    buttons: Vec<Button>,
    joltage_requirements: Vec<i64>,
    pub presses_during_reduction: i64,
}

impl MachineP2 {
    fn button_max_presses(&self) -> Vec<i64> {
        self.buttons
            .iter()
            .map(|button| {
                button
                    .indicator_indexes
                    .iter()
                    .map(|index| self.joltage_requirements[*index])
                    .min()
                    .unwrap()
            })
            .collect::<Vec<i64>>()
    }

    fn remove_index(&mut self, index_to_remove: usize) {
        let mut original_indexes: Vec<usize> = (0..self.joltage_requirements.len()).collect();
        original_indexes.retain(|i| *i != index_to_remove);
        assert_eq!(original_indexes.len(), self.joltage_requirements.len() - 1);

        for button in self.buttons.iter_mut() {
            button.indicator_indexes.remove(&index_to_remove);
        }

        assert_eq!(self.joltage_requirements[index_to_remove], 0);
        self.joltage_requirements.remove(index_to_remove);

        // change the indexes
        for button in self.buttons.iter_mut() {
            let new_indices: HashSet<usize> =
                HashSet::from_iter(button.indicator_indexes.iter().map(|old_index| {
                    assert_ne!(*old_index, index_to_remove);
                    original_indexes
                        .iter()
                        .position(|i| *i == *old_index)
                        .unwrap()
                }));
            button.indicator_indexes = new_indices
        }
    }

    fn check_joltage_combination(&self, combination: &Vec<i64>) -> bool {
        let mut counters: Vec<i64> = self.joltage_requirements.clone();
        for (button_index, times_to_press) in combination.into_iter().enumerate() {
            if button_index >= self.buttons.len() {
                println!(
                    "failure checking combination {:?}\n  against self: {:?}",
                    combination, self
                );
            }
            for counter_index in self.buttons[button_index].indicator_indexes.iter() {
                counters[*counter_index] -= times_to_press;
            }
        }

        return !counters.iter().any(|counter| *counter != 0);
    }

    // what if I look for counters that are only affected by one button
    // and press that button however many times
    // then I can just remove that counter from the machine, and reduce the targets accordingly
    // remember to remove the index from the button that affects it, and recalculate the indexes
    // then repeat?
    // if there are none, do the classic brute force

    /// remove counters that are only affected by a single button
    pub fn reduce_single_counters(&mut self) {
        while self.remove_single_counter() {
            println!("reduced")
        }
    }

    /// remove the first counter that is only affected by a single button
    /// return true if a counter was removed
    fn remove_single_counter(&mut self) -> bool {
        for index in 0..self.joltage_requirements.len() {
            let buttons_affecting_current_index: Vec<&mut Button> = self
                .buttons
                .iter_mut()
                .filter(|button| button.indicator_indexes.contains(&index))
                .collect();
            if buttons_affecting_current_index.len() == 1 {
                let button = buttons_affecting_current_index.into_iter().next().unwrap();
                let removed_counter_target = self.joltage_requirements[index];
                for indicator_index in button.indicator_indexes.iter() {
                    // assuming the puzzle is possible, this shouldn't go below zero:
                    self.joltage_requirements[*indicator_index] -= removed_counter_target;
                    assert!(self.joltage_requirements[*indicator_index] >= 0)
                }
                self.presses_during_reduction += removed_counter_target;
                self.remove_index(index);
                return true;
            }
        }
        return false;
    }

    // fn split_once(mut self) -> Vec<Self> {
    //     // find all the indexes that share a loop
    // }

    // fn split(mut self) -> Vec<Self> {
    //     let mut machines = vec![];
    //     while !self.buttons.is_empty() {
    //         let mut new_machine_buttons = vec![self.buttons.pop().unwrap()]; // still with the original indices
    //         loop {
    //             let still_to_add: Vec<Button> = self
    //                 .buttons
    //                 .extract_if(.., |button| {
    //                     !button.indicator_indexes.is_disjoint(&HashSet::from_iter(
    //                         new_machine_buttons
    //                             .iter()
    //                             .flat_map(|button| button.indicator_indexes.iter().cloned()),
    //                     ))
    //                 })
    //                 .collect();

    //             if still_to_add.is_empty() {
    //                 break;
    //             }
    //             new_machine_buttons.extend(still_to_add);
    //         }

    //         // map the indexes somehow
    //         let new_machine_original_indices: Vec<usize> = HashSet::<usize>::from_iter(
    //             new_machine_buttons
    //                 .iter()
    //                 .flat_map(|button| button.indicator_indexes.iter().cloned()),
    //         )
    //         .into_iter()
    //         .collect();

    //         for button in new_machine_buttons.iter_mut() {
    //             let new_indexes: HashSet<usize> =
    //                 HashSet::from_iter(button.indicator_indexes.iter().map(|original_index| {
    //                     new_machine_original_indices
    //                         .iter()
    //                         .position(|elem| elem == original_index)
    //                         .unwrap()
    //                 }));
    //             button.indicator_indexes = new_indexes;
    //         }
    //         machines.push(MachineP2 {
    //             buttons: new_machine_buttons,
    //             joltage_requirements: new_machine_original_indices
    //                 .iter()
    //                 .map(|original_index| self.joltage_requirements[*original_index])
    //                 .collect(),
    //             presses_during_reduction: 0,
    //         })
    //     }

    //     machines[0].presses_during_reduction = self.presses_during_reduction;

    //     return machines;
    // }

    // returns the button that increases the most counters without going past the joltage targets
    // fn max_button(&self, counters: Vec<i64>) -> usize {
    //     counter_doesnt_overflow |button: &Button| {
    //         let counter_copy = counters.clone();
    //     }

    //     self.buttons.iter().enumerate().filter()
    // }

    fn joltage_min_presses(&self) -> i64 {
        // combinations of numbers from 0..max_target_val_button_affects
        let mut combination: Vec<i64> = vec![0; self.buttons.len()];
        let mut min_presses_seen: i64 = i64::MAX;
        'outer: loop {
            if self.check_joltage_combination(&combination) {
                min_presses_seen = min(combination.iter().sum(), min_presses_seen)
            }
            combination[0] += 1;
            let mut overflow_check_index = 0;
            loop {
                if combination[overflow_check_index]
                    > self.joltage_requirements[overflow_check_index]
                {
                    combination[overflow_check_index] = 0;
                    overflow_check_index += 1;
                    if overflow_check_index >= self.joltage_requirements.len() {
                        break 'outer;
                    }
                    combination[overflow_check_index] += 1;
                } else {
                    break;
                }
            }
        }
        return min_presses_seen;
    }

    pub fn num_naive_combinations(&self) -> u128 {
        self.joltage_requirements
            .iter()
            .map(|jr| *jr as u128)
            .product()
    }
}

#[derive(PartialEq, Debug)]
struct Machine {
    target_indicators: Vec<bool>,
    buttons: Vec<Button>,
}

impl Machine {
    fn new() -> Self {
        Self {
            target_indicators: vec![],
            buttons: vec![],
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

fn machine_parser<'a>() -> impl Parser<'a, &'a str, Machine> {
    target_indicator_parser()
        .then_ignore(whitespace())
        .then(buttons_parser())
        .then_ignore(whitespace())
        .then(joltage_requirements_parser())
        .map(|((target_indicators, buttons), _)| Machine {
            target_indicators,
            buttons,
        })
}

fn machine_p2_parser<'a>() -> impl Parser<'a, &'a str, MachineP2> {
    target_indicator_parser()
        .then_ignore(whitespace())
        .then(buttons_parser())
        .then_ignore(whitespace())
        .then(joltage_requirements_parser())
        .map(|((_, buttons), joltage_requirements)| MachineP2 {
            joltage_requirements,
            buttons,
            presses_during_reduction: 0,
        })
}

fn parse(input: &str) -> impl Iterator<Item = Machine> {
    input
        .lines()
        .map(|line| machine_parser().parse(line).unwrap())
}

fn parse_p2(input: &str) -> impl Iterator<Item = MachineP2> {
    input
        .lines()
        .map(|line| machine_p2_parser().parse(line).unwrap())
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

fn part_two(input: &str) -> i64 {
    let machines: Vec<MachineP2> = parse_p2(input)
        // .flat_map(|m| m.split())
        .collect();
    machines
        .into_par_iter()
        .map(|mut machine| {
            machine.reduce_single_counters();
            let maxes = machine.button_max_presses();
            let iterator = p2_combination_iterator(&maxes);

            let min: Vec<i64> = iterator
                .filter(|combination| machine.check_joltage_combination(combination))
                .min_by_key(|c| c.iter().sum::<i64>() + machine.presses_during_reduction)
                .unwrap();
            println!("found min for {:?}\n  min: {:?}", machine, min);
            min.iter().sum::<i64>() + machine.presses_during_reduction
        })
        .sum()
}

// this isn't in order :c
fn p2_combination_iterator(maxes: &Vec<i64>) -> impl Iterator<Item = Vec<i64>> {
    println!(
        "creating iterator over {} items (len {})",
        maxes.iter().map(|m| *m as u128).product::<u128>(),
        maxes.len()
    );
    maxes.iter().map(|max| 0..=*max).multi_cartesian_product()
}

// what if I iterate over combinations of buttons that achieve each counter target
// so for a target of 5 and 3 buttons affecting that target, there are 3^5 combinations for just those buttons
// and then for each combination, if it isn't already over the target for another counter, go to the next target and check combinations?
// but this is just another way of doing what I did splitting machines
// which didn't make a big difference

// number of button presses has min
// min is max of targets
// max is sum of targets
// could get a smaller bound on max by summing the min target affected by each button?

// if you take the highest target
// the buttons affecting that will have to be pressed by at least the difference to the next highest target

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            machine_parser()
                .parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")
                .unwrap(),
            Machine {
                target_indicators: vec![false, true, true, false],
                buttons: vec![
                    Button {
                        indicator_indexes: HashSet::from_iter(vec![3])
                    },
                    Button {
                        indicator_indexes: HashSet::from_iter(vec![1, 3])
                    },
                    Button {
                        indicator_indexes: HashSet::from_iter(vec![2])
                    },
                    Button {
                        indicator_indexes: HashSet::from_iter(vec![2, 3])
                    },
                    Button {
                        indicator_indexes: HashSet::from_iter(vec![0, 2])
                    },
                    Button {
                        indicator_indexes: HashSet::from_iter(vec![0, 1])
                    },
                ],
                // joltage_requirements: vec![3, 5, 4, 7],
                // presses_during_reduction: 0,
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
        let machine = machine_parser()
            .parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")
            .unwrap();
        assert!(machine.check_combination(&vec![0, 1, 2]))
    }

    #[test]
    fn test_machine_min_presses() {
        let machine = machine_parser()
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

    #[test]
    fn test_split() {
        let original_machine = machine_parser()
            .parse("[.##.] (3) (2,3) (2) (0,1) {3,5,4,7}")
            .unwrap();
        // panic!("{:?}", original_machine.split());
    }

    // #[test]
    // fn test_num_splits() {
    //     let input = read_to_string("input.txt").unwrap();
    //     let machines: Vec<MachineP2> = parse_p2(&input).collect();
    //     let num_machines = machines.len();
    //     let new_machines = machines
    //         .into_iter()
    //         .flat_map(|machine| machine.split())
    //         .count();
    //     // panic!("from {} to {} machines, difference {}", num_machines, new_machines, new_machines - num_machines)
    // }

    #[test]
    fn test_joltage_combination_check() {
        let machine = machine_p2_parser()
            .parse("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
            .unwrap();
        assert!(machine.check_joltage_combination(&vec![2, 5, 0, 5, 0]));
    }

    #[test]
    fn test_reduce_single_counters() {
        let input = read_to_string("input.txt").unwrap();
        let mut machines: Vec<MachineP2> = parse_p2(&input)
            // .flat_map(|m| m.split())
            .collect();
        println!(
            "max' before: {:?}",
            machines.iter().map(|m| m.num_naive_combinations()).max()
        );
        for machine in machines.iter_mut() {
            machine.reduce_single_counters();
        }

        println!(
            "max' after: {:?}",
            machines.iter().map(|m| m.num_naive_combinations()).max()
        );
    }

    #[test]
    fn part_two_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part_two(&input), 33)
    }

    #[test]
    fn part_two_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(part_two(&input), 0)
    }
}
