use std::fs::{read_to_string, File};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    read_to_string(file)
        .unwrap()
        .lines()
        .map(|line| {
            line.split(" ")
        })
}
