use std::collections::HashSet;

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord {
    pub fn distance(&self, other: &Coord) -> i64 {
        let x = (self.x - other.x);
        let y = (self.y - other.y);
        let z = (self.z - other.z);
        return (x * x) + (y * y) + (z * z);
    }
}

struct Circuit {
    junction_boxes: Vec<JunctionBox>,
}

impl Circuit {
    pub fn min_distance(&self, other: &Circuit) -> i64 {
        self.junction_boxes
            .iter()
            .map(|j| {
                other
                    .junction_boxes
                    .iter()
                    .map(|k| j.coord.distance(&k.coord))
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap()
    }

    pub fn connect(&mut self, mut other: Circuit) {
        self.junction_boxes.append(&mut other.junction_boxes);
    }
}

#[derive(Debug)]
struct JunctionBox {
    coord: Coord,
    connections: Vec<usize>,
}

/*

groups of coordinates by distance

some kind of bound on a group?
so that you can check the distance without iterating through all members of a group
but groups likely won't grow that large?
easier to iterate.

closest_distance function between two groups.
coircuit is a vec of coords


*/

fn parse(input: &str) -> impl Iterator<Item = Coord> {
    input.lines().map(|line| {
        let mut parts = line.split(",");
        Coord {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
            z: parts.next().unwrap().parse().unwrap(),
        }
    })
}

fn n_closest_not_connected(n: usize, items: impl Iterator<Item = Coord>) -> i64 {
    let mut circuits: Vec<Circuit> = items
        .map(|c| Circuit {
            junction_boxes: vec![JunctionBox {coord: c, connections: vec![]}],
        })
        .collect();

    let num_circuits = circuits.len();
    for _ in 0..n {
        let pair_iterator = circuits.iter().flat_map(|c| &c.junction_boxes).flat_map(|j_box| circuits.iter().flat_map(|c| &c.junction_boxes).map(move |a| (j_box, a)));
        let pairs: Vec<(usize, usize)> = (0..(num_circuits.pow(2)))
            .into_iter()
            .map(|i| (i / num_circuits, i % num_circuits))
            .filter(|(i, j)| i != j)
            // .filter(|(i, j)| )
            .collect();

        let closest_pair = pairs
            .iter()
            .min_by(|(i, j), (k, l)| {
                circuits[*i]
                    .min_distance(&circuits[*j])
                    .cmp(&circuits[*k].min_distance(&circuits[*l]))
            })
            .unwrap();

        let removed = circuits.remove(closest_pair.1);
        circuits[closest_pair.0].connect(removed);
    }

    circuits.sort_by_key(|c| c.junction_boxes.len());
    circuits.reverse();
    // eprintln!(
    //     "{:?}",
    //     circuits
    //         .iter()
    //         .map(|c| c.junction_boxes.len())
    //         .collect::<Vec<usize>>()
    // );
    return circuits
        .iter()
        .take(3)
        .map(|c| c.junction_boxes.len())
        .product::<usize>() as i64;
}

fn n_closest_not_directly_connected(
    n: usize,
    items: impl Iterator<Item = Coord>,
) -> Vec<JunctionBox> {
    let mut junction_boxes: Vec<JunctionBox> = items
        .map(|c| JunctionBox {
            coord: c,
            connections: vec![],
        })
        .collect();

    let num_boxes = junction_boxes.len();

    for _ in 0..n {
        let pairs = (0..(num_boxes.pow(2)))
            .into_iter()
            .map(|i| (i / num_boxes, i % num_boxes))
            .filter(|(i, j)| i != j)
            .filter(|(i, j)| {
                !(junction_boxes[*i].connections.contains(j)
                    || junction_boxes[*j].connections.contains(i))
            });

        let closest_pair = pairs
            .min_by(|(i, j), (k, l)| {
                junction_boxes[*i]
                    .coord
                    .distance(&junction_boxes[*j].coord)
                    .cmp(&junction_boxes[*k].coord.distance(&junction_boxes[*l].coord))
            })
            .unwrap();

        // eprintln!(
        //     "connecting {:?} and {:?}, distance {}",
        //     junction_boxes[closest_pair.0],
        //     junction_boxes[closest_pair.1],
        //     junction_boxes[closest_pair.0]
        //         .coord
        //         .distance(&junction_boxes[closest_pair.1].coord)
        // );

        junction_boxes[closest_pair.0]
            .connections
            .push(closest_pair.1);
        junction_boxes[closest_pair.1]
            .connections
            .push(closest_pair.0);
    }

    return junction_boxes;
}

fn size_of_circuits(boxes: Vec<JunctionBox>) -> impl Iterator<Item = usize> {
    let mut sets: Vec<HashSet<usize>> = vec![];
    for i in 0..boxes.len() {
        match sets.iter().position(|set| set.contains(&i)) {
            Some(set_index) => {
                let mut still_to_add = HashSet::new();
                still_to_add.extend(boxes[i].connections.clone());
                while !still_to_add.is_empty() {
                    let item = still_to_add.iter().next().unwrap();
                    sets[set_index].insert(*item);
                    still_to_add.extend(boxes[*item].connections.clone());

                    let old_still_to_add = still_to_add.clone();
                    for elem in old_still_to_add.intersection(&sets[set_index]) {
                        still_to_add.remove(&elem);
                    }
                }
            }
            None => {
                let mut new_circuit = HashSet::new();
                new_circuit.insert(i);

                let mut still_to_add = HashSet::new();
                still_to_add.extend(boxes[i].connections.clone());
                while !still_to_add.is_empty() {
                    let item = still_to_add.iter().next().unwrap();
                    new_circuit.insert(*item);
                    still_to_add.extend(boxes[*item].connections.clone());

                    let old_still_to_add = still_to_add.clone();
                    for elem in old_still_to_add.intersection(&new_circuit) {
                        still_to_add.remove(&elem);
                    }
                }

                sets.push(new_circuit);
            }
        }
    }

    return sets.into_iter().map(|s| s.iter().len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part_one_example() {
        let input = read_to_string("test_input.txt").unwrap();
        let mut circuit_sizes: Vec<usize> =
            size_of_circuits(n_closest_not_directly_connected(10, parse(&input))).collect();
        // assert_eq!(circuit_sizes.len(), 11);
        circuit_sizes.sort();
        circuit_sizes.reverse();
        eprintln!("{:?}", circuit_sizes);
        assert_eq!(circuit_sizes.into_iter().take(3).product::<usize>(), 40)
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("input.txt").unwrap();
        let mut circuit_sizes: Vec<usize> =
            size_of_circuits(n_closest_not_directly_connected(1000, parse(&input))).collect();
        circuit_sizes.sort();
        circuit_sizes.reverse();
        assert_eq!(circuit_sizes.into_iter().take(3).product::<usize>(), 0);
    }
}
