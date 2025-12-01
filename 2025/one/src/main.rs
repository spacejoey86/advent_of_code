use std::fs::read_to_string;

fn main() {
    println!("count: {}", part_two())
}

pub fn part_one() -> u32 {
    let mut dial = Dial::new();
    for line in read_to_string("input.txt").unwrap().lines() {
        dial.rotate(Dial::parse_rotate(line))
    }
    return dial.times_left_at_zero()
}

pub fn part_two() -> u32 {
    let mut dial = Dial::new();
    for line in read_to_string("input.txt").unwrap().lines() {
        dial.rotate_part2(Dial::parse_rotate(line))
    }
    return dial.times_left_at_zero()
}

struct Dial {
    pointing: i32,
    times_left_at_zero: u32,
}

impl Dial {
    pub fn new() -> Self {
        return Self {
            pointing: 50,
            times_left_at_zero: 0,
        }
    }

    fn rotate(&mut self, distance: i32) {
        self.pointing = (self.pointing + distance) % 100;
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

    pub fn parse_rotate(line: &str) -> i32 {
        let distance: i32 = line[1..].parse().unwrap();
        match line.chars().nth(0) {
            Some('L') => -distance,
            Some('R') => distance,
            _ => panic!("failed to parse line: {}", line)
        }
    }

    pub fn times_left_at_zero(&self) -> u32 {
        return self.times_left_at_zero;
    }
}
