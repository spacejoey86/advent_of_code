use five::run;
use std::{fs::read_to_string, hint::black_box};

pub fn main() {
    let inp = black_box(read_to_string("input.txt").unwrap());
    for _ in 0..1_000_000 {
        black_box(run(&inp));
    }
}
