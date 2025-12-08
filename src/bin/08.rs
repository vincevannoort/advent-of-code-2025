use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::{collections::HashSet, hash::Hash};

advent_of_code::solution!(8);

fn parse(input: &str) -> Vec<(i32, i32, i32)> {
    let junction_boxes: Vec<(i32, i32, i32)> = input
        .lines()
        .map(|line| {
            let Some((x, y, z)) = line.splitn(3, ",").collect_tuple() else {
                panic!("cannot parse line")
            };
            (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
        })
        .collect();

    junction_boxes
}

fn euclidean_distance_3d(p1: (i32, i32, i32), p2: (i32, i32, i32)) -> f64 {
    let dx: f64 = (p1.0 - p2.0).into();
    let dy: f64 = (p1.1 - p2.1).into();
    let dz: f64 = (p1.2 - p2.2).into();
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn find_closest_pairs(
    junction_boxes: &Vec<(i32, i32, i32)>,
    pairs: usize,
) -> Vec<(&(i32, i32, i32), &(i32, i32, i32), f64)> {
    junction_boxes
        .iter()
        .combinations(2)
        .map(|pair| {
            let a = pair[0];
            let b = pair[1];
            (a, b, euclidean_distance_3d(*a, *b))
        })
        .sorted_by_key(|(_, _, distance)| OrderedFloat(*distance))
        .take(pairs)
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let junction_boxes = parse(input);
    let pairs = if junction_boxes.len() == 1000 {
        1000
    } else {
        10
    };

    let shortest_connections = find_closest_pairs(&junction_boxes, pairs);

    let mut circuits: Vec<HashSet<(i32, i32, i32)>> = vec![];

    for (a, b, _) in shortest_connections {
        let connected_circuits = circuits
            .iter_mut()
            .enumerate()
            .filter(|(_, circuit)| circuit.contains(a) || circuit.contains(b))
            .map(|(index, _)| index)
            .collect_vec();

        // create a new circuit
        if connected_circuits.is_empty() {
            let new_circuit = HashSet::from_iter(vec![*a, *b].into_iter());
            circuits.push(new_circuit);
            continue;
        }

        // extend existing circuit
        let first_circuit_index = *connected_circuits.first().unwrap();
        if connected_circuits.len() == 1 {
            let circuit = circuits.get_mut(first_circuit_index).unwrap();
            circuit.insert(*a);
            circuit.insert(*b);
            continue;
        }

        // combine circuits
        let second_circuit_index = *connected_circuits.last().unwrap();
        if connected_circuits.len() == 2 {
            let first_circuit = circuits.get(first_circuit_index).cloned().unwrap();
            let second_circuit = circuits.get(second_circuit_index).cloned().unwrap();
            circuits.retain(|circuit| *circuit != first_circuit && *circuit != second_circuit);
            let mut combined_circuit: HashSet<(i32, i32, i32)> = HashSet::new();
            combined_circuit.extend(&first_circuit);
            combined_circuit.extend(&second_circuit);
            circuits.push(combined_circuit);
        }
    }

    let largest_circuits = circuits
        .iter()
        .map(|circuit| circuit.len())
        .sorted()
        .rev()
        .collect_vec();

    let result = largest_circuits
        .into_iter()
        .take(3)
        .reduce(|acc, val| acc * val)
        .unwrap();

    Some(result.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closest_connections() {
        let junction_boxes = parse(&advent_of_code::template::read_file("examples", DAY));
        let mut shortest_connections = find_closest_pairs(&junction_boxes, 10).into_iter();

        let first = shortest_connections.next().unwrap();
        assert_eq!((first.0, first.1), (&(162, 817, 812), &(425, 690, 689)));
        let second = shortest_connections.next().unwrap();
        assert_eq!((second.0, second.1), (&(162, 817, 812), &(431, 825, 988)));
        let third = shortest_connections.next().unwrap();
        assert_eq!((third.0, third.1), (&(906, 360, 560), &(805, 96, 715)));
        let fourth = shortest_connections.next().unwrap();
        assert_eq!((fourth.0, fourth.1), (&(431, 825, 988), &(425, 690, 689)));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
