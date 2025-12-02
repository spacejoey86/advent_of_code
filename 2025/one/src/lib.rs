pub fn part_one(input: &str) -> i32 {
    let mut dial = Dial::new();
    for line in input.lines() {
        dial.rotate(Dial::parse_rotate(line))
    }
    return dial.times_left_at_zero()
}

pub fn part_two(input: &str) -> i32 {
    let mut dial = Dial::new();
    for line in input.lines() {
        dial.rotate_part2(Dial::parse_rotate(line))
    }
    return dial.times_left_at_zero()
}

struct Dial {
    pointing: i32,
    times_left_at_zero: i32,
}

impl Dial {
    pub const DIAL_SIZE: i32 = 100;
    pub fn new() -> Self {
        return Self {
            pointing: 50,
            times_left_at_zero: 0,
        }
    }

    fn rotate(&mut self, distance: i32) {
        self.pointing = (self.pointing + distance) % Dial::DIAL_SIZE;
        if self.pointing == 0 {
            self.times_left_at_zero += 1;
        }
    }

    fn rotate_part2(&mut self, distance: i32) {
        let direction: i32 = distance.signum();
        for _ in 0..distance.abs() {
            self.rotate(direction)
        }
    }

    fn rotate_part2_fast(&mut self, distance: i32) {
        // technically there should be an early return if distance=0, but that is never in the input
        // because my logic would give the wrong output
        while self.pointing >= Dial::DIAL_SIZE {
            self.pointing -= Dial::DIAL_SIZE;
            self.times_left_at_zero += 1;
        }
        if distance > 0 {
            self.pointing += distance;
            self.times_left_at_zero += self.pointing / Dial::DIAL_SIZE;
        } else if distance < 0 {
            self.pointing += distance;
            self.times_left_at_zero += distance.abs() / -Dial::DIAL_SIZE;
        }
        self.pointing = self.pointing % Dial::DIAL_SIZE;
        // self.pointing = (self.pointing + distance) % Dial::DIAL_SIZE;
        // if self.pointing == 0 {
        //     self.times_left_at_zero += 1;
        // }
    }

    pub fn parse_rotate(line: &str) -> i32 {
        let distance: i32 = line[1..].parse().unwrap();
        match line.chars().nth(0) {
            Some('L') => -distance,
            Some('R') => distance,
            _ => unreachable!("failed to parse line: {}", line)
        }
    }

    pub fn times_left_at_zero(&self) -> i32 {
        return self.times_left_at_zero;
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn part_one_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(part_one(&input), 1177)
    }

    #[test]
    fn part_one_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part_one(&input), 3)
    }

    #[test]
    fn part_two_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(part_two(&input), 6768)
    }

    #[test]
    fn part_two_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part_two(&input), 6)
    }
}