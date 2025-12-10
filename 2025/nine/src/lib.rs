use std::cmp::{max, min};

use itertools::{Combinations, Itertools};

// top left is zero zero
#[derive(Clone)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    pub fn area(&self, other: &Coord) -> i64 {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }

    // assumes the coordinates share an ordinate
    // as the adjacent coords in the input do
    pub fn direction(&self, other: &Coord) -> Direction {
        if self.x == other.x {
            // either north or south
            if other.y > self.y {
                return Direction::South;
            } else {
                return Direction::North;
            }
        } else {
            if other.x > self.x {
                return Direction::East;
            } else {
                return Direction::West;
            }
        }
    }

    pub fn inside(&self, corner1: &Coord, corner2: &Coord) -> bool {
        ((self.x > corner1.x && self.x < corner2.x) || (self.x < corner1.x && self.x > corner2.x))
            && ((self.y > corner1.y && self.y < corner2.y)
                || (self.y < corner1.y && self.y > corner2.y))
    }

    // does the line between end_1 and end_2 enter the rectangle between corner1 and corner2
    pub fn line_inside(corner1: &Coord, corner2: &Coord, end_1: &Coord, end_2: &Coord) -> bool {
        if end_1.x == end_2.x {
            // vertical line
            if (corner1.x >= end_1.x && corner2.x >= end_1.x) || (corner1.x <= end_1.x && corner2.x <= end_1.x) {
                // no x overlap
                return false;
            }

            // remember, origin is top left
            let top_end = min(end_1.y, end_2.y);
            let bottom_end = max(end_1.y, end_2.y);

            let top_of_rect = min(corner1.y, corner2.y);
            let bottom_of_rect = max(corner1.y, corner2.y);

            return ! ((top_end >= bottom_of_rect) || (bottom_end <= top_of_rect))
        } else {
            // horizontal line
            if (corner1.y >= end_1.y && corner2.y >= end_1.y) || (corner1.y <= end_1.y && corner2.y <= end_1.y) {
                // no y overlap
                return false;
            }

            // back to the correct origin
            let left_end = min(end_1.x, end_2.x);
            let right_end = max(end_1.x, end_2.x);
            let left_of_rect = min(corner1.x, corner2.x);
            let right_of_rect = max(corner1.x, corner2.x);
            return ! ((left_end >= right_of_rect) || (right_end <= left_of_rect))
        }
    }
}

pub fn parse(input: &str) -> impl Iterator<Item = Coord> {
    input.lines().map(|line| {
        let parts = line.split_once(",").unwrap();
        Coord {
            x: parts.0.parse().unwrap(),
            y: parts.1.parse().unwrap(),
        }
    })
}

pub fn pair_iterator(coords: &Vec<Coord>) -> impl Iterator<Item = (&Coord, &Coord)> {
    coords.iter().tuple_combinations()
}

pub fn part_one(input: &str) -> i64 {
    let coords = parse(input).collect();
    pair_iterator(&coords)
        .map(|(first_corner, second_corner)| first_corner.area(second_corner))
        .max()
        .unwrap()
}

// assumtions
// the shape is convex
// we are given that adjacent coordinates share a row or column

// so can I keep track of the biggest ordinate that could be a corner
// and also a value for the biggest valid rectangle found so far

// and change that based on the direction the next coordinate is

// specifically keeping track of the valid ordinates for every coordinate we already have

// maybe I can collapse multiple coordinates in the same direction into one update to the other_corners
// so that I can keep track better

// adjacent coordinates will always be valid corners of a thin rectangle

struct MaxValidOtherCorners {
    rg_area_top_left: Coord,
    rg_area_bottom_right: Coord,
}

// impl MaxValidOtherCorners {
//     fn update(&mut self, new_coordinate: &Coord, direction: &Direction) {
//         match direction {
//             Direction::North => {
//                 self.rg_area_top_left.y = min(self.rg_area_top_left.y, )
//             },
//             Direction::East => todo!(),
//             Direction::South => todo!(),
//             Direction::West => todo!(),
//         }
//     }
// }

enum Direction {
    North,
    East,
    South,
    West,
}

// fn part_two(input: &str) -> i64 {
//     let coords: Vec<Coord> = parse(input).collect();
//     // let other_corners: Vec<MaxValidOtherCorners> = coords.iter()
//     //     .map(|coord| MaxValidOtherCorners {
//     //         rg_area_top_left: coord.clone(),
//     //         rg_area_bottom_right: coord.clone(),
//     //     })
//     //     .collect();

//     // iterate over edges between adjacent coordinates
//     // for from_coord_index in 0..coords.len() {
//     //     let from = &coords[from_coord_index];
//     //     let to = &coords[(from_coord_index + 1) % coords.len()];

//     //     // extend the other corners somehow?
//     // }

//     pair_iterator(&coords)
//         .filter(|(corner1, corner2)| {
//             !coords
//                 .iter()
//                 .any(|other_coord| other_coord.inside(corner1, corner2))
//         })
//         .map(|(corner1, corner2)| corner1.area(corner2))
//         .max()
//         .unwrap()
// }

fn compress_coords(original_coords: &Vec<Coord>) -> Vec<(usize, i64, i64)> {
    let mut indexes: Vec<(usize, i64, i64)> = (0..(original_coords.len()))
        .into_iter()
        .map(|index| (index, 0, 0))
        .collect();

    // sort by x ordinate
    indexes.sort_by_key(|(index, _, _)| original_coords[*index].x);
    // insert gaps and collapse identical ordinates
    let mut prev_original_x_val = original_coords[indexes[0].0].x;
    let mut current_compressed_ord = 0;
    for (original_coord_index, compressed_x, _) in indexes.iter_mut().skip(1) {
        let current_original_x_val = original_coords[*original_coord_index].x;

        if prev_original_x_val != current_original_x_val {
            current_compressed_ord += 2;
            *compressed_x = current_compressed_ord;
        }

        prev_original_x_val = current_original_x_val;
    }

    // now repeat to fill in the compressed y values
    indexes.sort_by_key(|(index, _, _)| original_coords[*index].y);
    let mut prev_original_y_val = original_coords[indexes[0].0].y;
    let mut current_compressed_ord = 0;
    for (original_coord_index, _, compressed_y) in indexes.iter_mut().skip(1) {
        let current_original_y_val = original_coords[*original_coord_index].y;

        if prev_original_y_val != current_original_y_val {
            current_compressed_ord += 2;
            *compressed_y = current_compressed_ord;
        }

        prev_original_y_val = current_original_y_val;
    }

    return indexes;
}


// iterate over all pairs of coordinates
// keep track of the max area so far
// if the pair of coordinates has a bigger area
// then check for overlaps against all line segments (adjacent in the array, so this check is O(n) ? )
// so under O(n^3) ?

fn part_two(input: &str) -> i64 {
    let coords: Vec<Coord> = parse(input).collect();
    // let other_corners: Vec<MaxValidOtherCorners> = coords.iter()
    //     .map(|coord| MaxValidOtherCorners {
    //         rg_area_top_left: coord.clone(),
    //         rg_area_bottom_right: coord.clone(),
    //     })
    //     .collect();

    // iterate over edges between adjacent coordinates
    // for from_coord_index in 0..coords.len() {
    //     let from = &coords[from_coord_index];
    //     let to = &coords[(from_coord_index + 1) % coords.len()];

    //     // extend the other corners somehow?
    // }

    let mut max_area = 0;
    for (corner_1, corner_2) in pair_iterator(&coords) {
        let new_area = corner_1.area(corner_2);
        if new_area <= max_area {
            continue;
        }
        if coords.iter().circular_tuple_windows::<(&Coord, &Coord)>().any(|(end_1, end_2)| Coord::line_inside(corner_1, corner_2, end_1, end_2)) {
            continue;
        }
        max_area = new_area;
    };

    return max_area;
    // pair_iterator(&coords)
    //     .filter(|(corner1, corner2)| {
    //         !coords
    //             .iter()
    //             .any(|other_coord| other_coord.inside(corner1, corner2))
    //     })
    //     .map(|(corner1, corner2)| corner1.area(corner2))
    //     .max()
    //     .unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn part_one_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part_one(&input), 50)
    }

    #[test]
    fn part_one_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(part_one(&input), 4771532800)
    }

    #[test]
    fn part_two_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part_two(&input), 24)
    }

    #[test]
    fn part_two_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(part_two(&input), 1544362560)
    }
}
