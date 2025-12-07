pub fn part_one(input: &str) -> i64 {
    let mut lines = input.lines();
    let mut num_splits = 0;

    let mut beams: Vec<bool> = lines
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|char| *char == b'S')
        .collect();

    for line_string in lines {
        let mut new_beams: Vec<bool> = vec![false; beams.len()];
        for (index, char) in line_string.as_bytes().iter().enumerate() {
            if beams[index] {
                if *char == b'^' {
                    num_splits += 1;
                    new_beams[index + 1] = true;
                    new_beams[index - 1] = true;
                } else {
                    new_beams[index] = true;
                }
            }
        }
        beams = new_beams;
    }

    return num_splits;
}

pub fn part_two(input: &str) -> i64 {
    let mut lines = input.lines();

    // the number of timelines a particle could be at each index
    let mut timelines: Vec<i64> = lines
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|char| (*char == b'S') as i64)
        .collect();

    for line_string in lines {
        let mut new_timelines: Vec<i64> = vec![0; timelines.len()];
        for (index, char) in line_string.as_bytes().iter().enumerate() {
            if timelines[index] > 0 {
                if *char == b'^' {
                    new_timelines[index + 1] += timelines[index];
                    new_timelines[index - 1] += timelines[index];
                } else {
                    new_timelines[index] += timelines[index];
                }
            }
        }
        timelines = new_timelines;
    }

    return timelines.into_iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn part_one_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part_one(&input), 21)
    }

    #[test]
    fn part_one_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(part_one(&input), 1605)
    }

    #[test]
    fn cast_bool_to_i64() {
        assert_eq!(true as i64, 1);
    }

    #[test]
    fn part_two_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part_two(&input), 40)
    }

    #[test]
    fn part_two_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(part_two(&input), 29893386035180)
    }
}
