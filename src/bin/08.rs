use itertools::Itertools;
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

pub fn part_one(input: &str) -> Option<u64> {
    let junction_boxes = parse(input);
    let take_n_shortest_connections = if junction_boxes.len() == 1000 {
        1000
    } else {
        10
    };

    let shortest_connections = junction_boxes
        .iter()
        .enumerate()
        // find closest connection for every box
        .map(|(i, junction_box_i)| {
            let (closest_junction_box, distance) = junction_boxes
                .iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .map(|(_, junction_box_j)| {
                    (
                        junction_box_j,
                        euclidean_distance_3d(*junction_box_i, *junction_box_j),
                    )
                })
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();
            (junction_box_i, closest_junction_box, distance)
        })
        // sort by distance
        .sorted_by(|(_, _, distance_a), (_, _, distance_b)| {
            distance_a.partial_cmp(distance_b).unwrap()
        })
        .take(take_n_shortest_connections)
        // TODO: use less hacky approach here...
        .map(|(a, b, d)| if a < b { (a, b, d) } else { (b, a, d) })
        .unique_by(|(a, b, _)| (*a, *b))
        .collect_vec();

    let mut circuits: Vec<HashSet<(i32, i32, i32)>> = vec![];

    dbg!(&shortest_connections);

    for (a, b, _) in shortest_connections {
        let circuit = circuits
            .iter_mut()
            .find(|circuit| circuit.contains(a) || circuit.contains(b));

        // create a new circuit
        if circuit.is_none() {
            let new_circuit = HashSet::from_iter(vec![*a, *b].into_iter());
            circuits.push(new_circuit);
            continue;
        }

        // extend existing circuit
        if let Some(circuit) = circuit {
            circuit.insert(*a);
            circuit.insert(*b);
        }
    }

    let largest_circuits = circuits
        .iter()
        .map(|circuit| circuit.len())
        .sorted()
        .rev()
        .take(3)
        .collect_vec();

    let result = largest_circuits
        .into_iter()
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
    fn test_euclid_distance_3d() {
        assert_eq!(
            euclidean_distance_3d((1, 1, 1,), (2, 2, 2)),
            1.7320508075688772f64
        );
    }

    #[test]
    fn test_closest_connections() {
        // 162,817,812 & 425,690,689 -> circuit of 2
        // 162,817,812 & 431,825,988 -> circuit of 3
        // 805,96,715 & 906,360,560 -> circuit of 3 + circuit of 2
        // 431,825,988 & 425,690,689 -> ...
    }

    #[test]
    fn test_example1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_example2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    // #[test]
    // fn test_example3() {
    //     let result = part_one(&advent_of_code::template::read_file_part(
    //         "examples", DAY, 3,
    //     ));
    //     assert_eq!(result, Some(6));
    // }

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
