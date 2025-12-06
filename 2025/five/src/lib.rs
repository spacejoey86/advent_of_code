use std::{
    cmp::{max, min},
    collections::BinaryHeap,
};

use smallvec::SmallVec;

#[derive(Debug, Clone)]
pub struct Range {
    pub lower: i64,
    pub upper: i64,
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.upper == other.upper
    }
}
impl Eq for Range {}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.upper.cmp(&other.upper)
    }
}

impl Range {
    pub fn contains(&self, ingredient: &i64) -> bool {
        return *ingredient >= self.lower && *ingredient <= self.upper;
    }

    pub fn into_iter(self) -> impl Iterator<Item = i64> {
        return self.lower..=self.upper;
    }

    pub fn count(&self) -> i64 {
        return (self.upper as i64 - self.lower as i64) + 1;
    }

    pub fn num_overlap(&self, other: &Range) -> i64 {
        if (self.upper < other.lower) || (self.lower > other.upper) {
            return 0;
        }
        let overlap_range = Range {
            lower: max(self.lower, other.lower),
            upper: min(self.upper, other.upper),
        };
        return overlap_range.count();
    }

    // remove the other range from self, potentially returning multiple ranges
    pub fn remove_range(self, other: &Range) -> impl Iterator<Item = Range> {
        if (self.upper < other.lower) || (self.lower > other.upper) {
            // no overlap
            return vec![self].into_iter();
        } else if self.lower >= other.lower && self.upper <= other.upper {
            // whole range overlaps
            return vec![].into_iter();
        } else if self.lower < other.lower && self.upper > other.upper {
            // self contains other entirely, with a range on either side
            return vec![
                Range {
                    lower: self.lower,
                    upper: other.lower - 1,
                },
                Range {
                    lower: other.upper + 1,
                    upper: self.upper,
                },
            ]
            .into_iter();
        } else if self.lower < other.lower {
            // other overlaps upper bound
            return vec![Range {
                lower: self.lower,
                upper: other.lower - 1,
            }]
            .into_iter();
        } else {
            // other overlaps lower bound
            return vec![Range {
                lower: other.upper + 1,
                upper: self.upper,
            }]
            .into_iter();
        }
    }

    // returns false if the ranges overlap, and so other should be removed
    pub fn add_range(&mut self, other: &Range) -> bool {
        if (self.upper < other.lower) || self.lower > other.upper {
            // no overlap
            true
        } else {
            self.lower = min(self.lower, other.lower);
            self.upper = max(self.upper, other.upper);
            false
        }
    }
}

pub fn parse(input: &str) -> (Vec<Range>, Vec<i64>) {
    let ranges = input
        .lines()
        .take_while(|line| *line != "")
        .map(|line| {
            let (lower, upper) = line.split_once("-").unwrap();
            Range {
                lower: lower.parse().unwrap(),
                upper: upper.parse().unwrap(),
            }
        })
        .collect();

    let available = input
        .lines()
        .skip_while(|line| *line != "")
        .skip(1)
        .map(|line| line.parse().unwrap())
        .collect();

    return (ranges, available);
}

pub fn part_one(input: &str) -> usize {
    let (ranges, available) = parse(input);

    available
        .into_iter()
        .filter(|ingredient| ranges.iter().any(|range| range.contains(ingredient)))
        .count()
}

pub fn part_two_slow(input: &str) -> usize {
    let (mut ranges, _) = parse(input);

    let mut count = 0;
    while ranges.len() > 0 {
        let range = ranges.pop().unwrap();
        for ingredient in range.into_iter() {
            if !ranges.iter().any(|range| range.contains(&ingredient)) {
                count += 1;
            }
        }
    }
    return count;
}

pub fn part_two(input: &str) -> i64 {
    let (mut ranges, _) = parse(input);

    let mut count = 0;
    while ranges.len() > 0 {
        let range = ranges.pop().unwrap();
        ranges = ranges
            .into_iter()
            .flat_map(|r| r.remove_range(&range))
            .collect();
        count += range.count()
    }
    return count;
}

// pub fn part_two_faster(input: &str) -> i64 {
//     let (mut ranges, _) = parse(input);
//     let not_overlapping_ranges = ranges.
// }

pub fn run_old(input: &str) -> i64 {
    let (mut ranges, _) = parse(input);
    let mut count = 0;
    while ranges.len() > 0 {
        let mut range = ranges.pop().unwrap();
        ranges = ranges.into_iter().filter(|r| range.add_range(r)).collect();
        count += range.count();
    }
    return count;
}

pub fn remove_overlaps(mut ranges: Vec<Range>) -> Vec<Range> {
    let mut new_ranges = vec![];
    while ranges.len() > 0 {
        let mut range = ranges.pop().unwrap();
        eprintln!("processing: {:?}", range);
        ranges = ranges.into_iter().filter(|r| range.add_range(r)).collect();
        eprintln!("here: {:?}", ranges);
        new_ranges.push(range);
    }
    return new_ranges;
}

pub fn parse_to_heap(input: &str) -> BinaryHeap<Range> {
    let mut heap = BinaryHeap::new();
    let _: Vec<()> = input
        .lines()
        .take_while(|line| *line != "")
        .map(|line| {
            let (lower, upper) = line.split_once("-").unwrap();
            heap.push(Range {
                lower: lower.parse().unwrap(),
                upper: upper.parse().unwrap(),
            })
        })
        .collect();
    return heap;
}

pub fn parse_to_heap_faster(input: &str) -> BinaryHeap<Range> {
    let mut heap = BinaryHeap::with_capacity(256);
    let mut bytes = input.bytes().peekable();
    loop {
        let mut lower: i64 = 0;
        while let char = unsafe { bytes.next().unwrap_unchecked() }
            && char != b'-'
        {
            lower = lower * 10 + char as i64 - b'0' as i64
        }
        let mut upper: i64 = 0;
        while let char = unsafe { bytes.next().unwrap_unchecked() }
            && char != b'\n'
        {
            upper = upper * 10 + char as i64 - b'0' as i64
        }
        heap.push(Range { lower, upper });
        if *unsafe { bytes.peek().unwrap_unchecked() } == b'\n' {
            break;
        }
    }

    return heap;
}

pub fn parse_to_smallvec_sorted(input: &str) -> SmallVec<[Range; 256]> {
    let mut ranges = SmallVec::with_capacity(256);
    let mut bytes = input.bytes().peekable();
    loop {
        let mut lower: i64 = 0;
        while let char = unsafe { bytes.next().unwrap_unchecked() }
            && char != b'-'
        {
            lower = lower * 10 + char as i64 - b'0' as i64
        }
        let mut upper: i64 = 0;
        while let char = unsafe { bytes.next().unwrap_unchecked() }
            && char != b'\n'
        {
            upper = upper * 10 + char as i64 - b'0' as i64
        }

        // ranges.push(Range { lower, upper });
        match ranges.binary_search_by(|r: &Range| r.upper.cmp(&upper)) {
            Ok(i) => {
                // upper bounds overlap, combine into one range with the min of lower bounds
                unsafe { ranges.get_unchecked_mut(i) }.lower =
                    min(unsafe { ranges.get_unchecked(i) }.lower, lower);
            }
            Err(i) => {
                ranges.insert(i, Range { lower, upper });
            }
        }

        if *unsafe { bytes.peek().unwrap_unchecked() } == b'\n' {
            break;
        }
    }

    return ranges;
}

pub fn parse_to_smallvec_then_sort(input: &str) -> SmallVec<[Range; 256]> {
    let mut ranges = SmallVec::with_capacity(256);
    let mut bytes = input.bytes().peekable();
    loop {
        let mut lower: i64 = 0;
        while let char = unsafe { bytes.next().unwrap_unchecked() }
            && char != b'-'
        {
            lower = lower * 10 + char as i64 - b'0' as i64
        }
        let mut upper: i64 = 0;
        while let char = unsafe { bytes.next().unwrap_unchecked() }
            && char != b'\n'
        {
            upper = upper * 10 + char as i64 - b'0' as i64
        }

        ranges.push(Range { lower, upper });

        if *unsafe { bytes.peek().unwrap_unchecked() } == b'\n' {
            break;
        }
    }
    ranges.sort_unstable_by_key(|r: &Range| r.upper);

    return ranges;
}

pub fn parse_to_vec_sorted(input: &str) -> Vec<Range> {
    let mut ranges = Vec::with_capacity(128);
    let mut bytes = input.bytes().peekable();
    loop {
        let mut lower: i64 = 0;
        while let char = unsafe { bytes.next().unwrap_unchecked() }
            && char != b'-'
        {
            lower = lower * 10 + char as i64 - b'0' as i64
        }
        let mut upper: i64 = 0;
        while let char = unsafe { bytes.next().unwrap_unchecked() }
            && char != b'\n'
        {
            upper = upper * 10 + char as i64 - b'0' as i64
        }

        // ranges.push(Range { lower, upper });
        match ranges.binary_search_by(|r: &Range| r.upper.cmp(&upper)) {
            Ok(i) => {
                // upper bounds overlap, combine into one range with the min of lower bounds
                unsafe { ranges.get_unchecked_mut(i) }.lower =
                    min(unsafe { ranges.get_unchecked(i) }.lower, lower);
            }
            Err(i) => {
                ranges.insert(i, Range { lower, upper });
            }
        }

        if *unsafe { bytes.peek().unwrap_unchecked() } == b'\n' {
            break;
        }
    }

    return ranges;
}

pub fn parse_to_vec_then_sort(input: &str) -> Vec<Range> {
    let mut ranges = Vec::with_capacity(256);
    let mut bytes = input.bytes().peekable();
    loop {
        let mut lower: i64 = 0;
        while let char = unsafe { bytes.next().unwrap_unchecked() }
            && char != b'-'
        {
            if char == b'\n' {
                break
            }
            lower = lower * 10 + char as i64 - b'0' as i64
        }
        let mut upper: i64 = 0;
        while let char = unsafe { bytes.next().unwrap_unchecked() }
            && char != b'\n'
        {
            upper = upper * 10 + char as i64 - b'0' as i64
        }

        ranges.push(Range { lower, upper });

        if *unsafe { bytes.peek().unwrap_unchecked() } == b'\n' {
            break;
        }
    }
    ranges.sort_unstable_by_key(|r| r.upper);

    return ranges;
}

pub fn run(input: &str) -> i64 {
    let mut sorted_ranges = parse_to_vec_then_sort(input);

    let mut count = 0;
    let mut current_range = unsafe { sorted_ranges.pop().unwrap_unchecked() };
    while let Some(new_range) = sorted_ranges.pop() {
        if new_range.upper < current_range.lower {
            // no overlap
            count += current_range.count();
            current_range = new_range;
        } else {
            // overlap
            current_range.lower = min(current_range.lower, new_range.lower)
        }
    }
    count += current_range.count();

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_parse() {
        let input = read_to_string("test_input.txt").unwrap();
        let (ranges, available) = parse(&input);
        assert_eq!(*ranges.get(0).unwrap(), Range { lower: 3, upper: 5 })
    }

    #[test]
    fn part_one_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part_one(&input), 3)
    }

    #[test]
    fn part_one_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(part_one(&input), 694)
    }

    #[test]
    fn part_two_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part_two(&input), 14)
    }

    #[test]
    fn part_two_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(part_two(&input), 352716206375547)
    }

    #[test]
    fn part_two_run_example_test() {
        let input = read_to_string("test_input.txt").unwrap();
        assert_eq!(run(&input), 14)
    }

    #[test]
    fn part_two_run_test() {
        let input = read_to_string("input.txt").unwrap();
        assert_eq!(run(&input), 352716206375547)
    }

    #[test]
    fn test_range_no_overlap() {
        let original_range = Range { lower: 3, upper: 5 };
        let ranges: Vec<Range> = original_range
            .clone()
            .remove_range(&Range { lower: 6, upper: 8 })
            .collect();
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0], original_range);
    }

    #[test]
    fn test_range_full_overlap() {
        let ranges: Vec<Range> = Range {
            lower: 8,
            upper: 10,
        }
        .remove_range(&Range {
            lower: 6,
            upper: 12,
        })
        .collect();
        assert_eq!(ranges.len(), 0);
    }

    #[test]
    fn test_range_entirely_contained() {
        let original_range = Range {
            lower: 15,
            upper: 25,
        };
        let ranges: Vec<Range> = original_range
            .remove_range(&Range {
                lower: 17,
                upper: 18,
            })
            .collect();
        assert_eq!(ranges.len(), 2);
        assert_eq!(
            ranges[0],
            Range {
                lower: 15,
                upper: 16
            }
        );
        assert_eq!(
            ranges[1],
            Range {
                lower: 19,
                upper: 25
            }
        );
    }

    #[test]
    fn test_range_lower_overlapped() {
        let ranges: Vec<Range> = Range {
            lower: 91,
            upper: 150,
        }
        .remove_range(&Range {
            lower: 82,
            upper: 110,
        })
        .collect();
        assert_eq!(ranges.len(), 1);
        assert_eq!(
            ranges[0],
            Range {
                lower: 111,
                upper: 150
            }
        );
    }

    #[test]
    fn test_combine_range_not_overlapping() {
        let mut range = Range { lower: 3, upper: 5 };

        assert_eq!(range.add_range(&Range { lower: 7, upper: 8 }), true);
        assert_eq!(range, Range { lower: 3, upper: 5 });
    }

    #[test]
    fn test_combine_range_other_bigger_overlap() {
        let mut range = Range { lower: 3, upper: 5 };
        assert_eq!(range.add_range(&Range { lower: 1, upper: 6 }), false);
        assert_eq!(range, Range { lower: 1, upper: 6 });
    }

    #[test]
    fn test_combine_ranges_adjacent() {
        let mut range = Range { lower: 3, upper: 5 };
        assert_eq!(range.add_range(&Range { lower: 6, upper: 8 }), true);
        assert_eq!(range, Range { lower: 3, upper: 5 })
    }

    #[test]
    fn test_combine_ranges_other_inside() {
        let mut range = Range { lower: 3, upper: 8 };
        assert_eq!(range.add_range(&Range { lower: 4, upper: 7 }), false);
        assert_eq!(range, Range { lower: 3, upper: 8 })
    }

    #[test]
    fn test_combine_ranges_edge_overlap() {
        let mut range = Range { lower: 3, upper: 5 };

        assert_eq!(range.add_range(&Range { lower: 5, upper: 8 }), false);
        assert_eq!(range, Range { lower: 3, upper: 8 });
    }

    #[test]
    fn test_combine_ranges_lower_overlaps() {
        let mut range = Range {
            lower: 3,
            upper: 10,
        };
        assert_eq!(false, range.add_range(&Range { lower: 2, upper: 7 }));
        assert_eq!(
            range,
            Range {
                lower: 2,
                upper: 10
            }
        );
    }

    #[test]
    fn test_combine_ranges_upper_overlaps() {
        let mut range = Range {
            lower: 3,
            upper: 10,
        };
        assert_eq!(
            false,
            range.add_range(&Range {
                lower: 7,
                upper: 10
            })
        );
        assert_eq!(
            range,
            Range {
                lower: 3,
                upper: 10
            }
        );
    }

    #[test]
    fn test_remove_overlaps() {
        let mut ranges = vec![
            Range { lower: 3, upper: 6 },
            Range { lower: 5, upper: 8 },
            Range { lower: 8, upper: 9 },
        ];
        let without_overlaps = remove_overlaps(ranges.clone());
        panic!("{:?}", without_overlaps);
    }

    #[test]
    fn test_parse_heap() {
        let heap = parse_to_heap("1-5\n9-12\n5-8\n\n");
        assert_eq!(
            heap.into_sorted_vec(),
            vec![
                Range { lower: 1, upper: 5 },
                Range { lower: 5, upper: 8 },
                Range {
                    lower: 9,
                    upper: 12
                }
            ]
        )
    }

    #[test]
    fn test_parse_heap_full_overlaps() {
        let heap = parse_to_heap("0-1000\n5-990\n10-900\n\n");
        assert_eq!(
            heap.into_sorted_vec(),
            vec![
                Range {
                    lower: 10,
                    upper: 900
                },
                Range {
                    lower: 5,
                    upper: 990
                },
                Range {
                    lower: 0,
                    upper: 1000
                }
            ]
        )
    }

    #[test]
    fn run_heap_full_overlaps() {
        assert_eq!(run("0-1000\n5-990\n10-900\n\n"), 1001)
    }
}
