use std::{fs::read_to_string, iter::zip};
use itertools::Itertools;

fn main() {
    println!("{}", sum_of_similarity_scores("input.txt"));
}

fn sum_of_sorted_differences(path: &str) -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = read_to_string(path)
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


    zip(left, right)
        .map(|(l, r)| {
            (l - r).abs()
        })
        .reduce(|running_total: i32, new| {
            running_total + new
        })
        .unwrap()
}

fn sum_of_similarity_scores(path: &str) -> i32 {
    let (left, right): (Vec<i32>, Vec<i32>) = read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let elements: Vec<&str> = line.split("   ").collect();
            (elements[0].parse::<i32>().unwrap(),
            elements[1].parse::<i32>().unwrap())
        })
        .unzip();

//    left.sort();
    let frequencies = right.into_iter().counts();


    left
        .into_iter()
        .map(|l| {
            l * *frequencies.get(&l).unwrap_or(&0) as i32
        })
        .reduce(|running_total: i32, new| {
            running_total + new
        })
        .unwrap()
}