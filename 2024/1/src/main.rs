use std::{fs::read_to_string, iter::zip};

fn main() {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let elements: Vec<&str> = line.split("   ").collect();
            (elements[0].parse::<i32>().unwrap(),
            elements[1].parse::<i32>().unwrap())
        })
        .unzip();

    left.sort();
    right.sort();


    let result = zip(left, right)
        .map(|(l, r)| {
            (l - r).abs()
        })
        .reduce(|running_total: i32, new| {
            running_total + new
        })
        .unwrap();

    println!("{}", result);
}
