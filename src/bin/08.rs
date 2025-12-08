use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::collections::HashSet;

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

fn update_circuits_with_connection(
    circuits: &mut Vec<HashSet<(i32, i32, i32)>>,
    a: &(i32, i32, i32),
    b: &(i32, i32, i32),
) {
    let index_a = circuits.iter().position(|c| c.contains(a));
    let index_b = circuits.iter().position(|c| c.contains(b));

    match (index_a, index_b) {
        // create new circuit
        (None, None) => {
            circuits.push(HashSet::from([*a, *b]));
        }
        // extend circuits
        (Some(i), None) => {
            circuits[i].insert(*b);
        }
        (None, Some(j)) => {
            circuits[j].insert(*a);
        }
        // same circuit
        (Some(i), Some(j)) if i == j => {}
        // combine circuits
        (Some(i), Some(j)) => {
            let second_circuit = circuits.remove(i.max(j));
            let first_circuit = circuits.get_mut(i.min(j)).unwrap();
            first_circuit.extend(second_circuit);
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let junction_boxes = parse(input);
    let pairs = match junction_boxes.len() {
        1000 => 1000,
        _ => 10,
    };

    let shortest_connections = find_closest_pairs(&junction_boxes, pairs);
    let mut circuits: Vec<HashSet<(i32, i32, i32)>> = vec![];

    for (a, b, _) in shortest_connections {
        update_circuits_with_connection(&mut circuits, a, b);
    }

    let result: usize = circuits.iter().map(|c| c.len()).k_largest(3).product();

    Some(result.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let junction_boxes = parse(input);

    let shortest_connections = find_closest_pairs(&junction_boxes, usize::MAX);
    let mut circuits: Vec<HashSet<(i32, i32, i32)>> = vec![];

    for (a, b, _) in shortest_connections {
        update_circuits_with_connection(&mut circuits, a, b);
        if circuits.len() == 1 && circuits.first().unwrap().len() == junction_boxes.len() {
            return Some(a.0 as u64 * b.0 as u64);
        }
    }

    panic!("did not complete to one big circuit")
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
        assert_eq!(result, Some(25272));
    }
}
