use advent_of_code::Location;
use itertools::Itertools;

advent_of_code::solution!(9);

fn parse(input: &str) -> Vec<Location> {
    let red_tiles: Vec<Location> = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap();
            Location {
                x: left.parse().unwrap(),
                y: right.parse().unwrap(),
            }
        })
        .collect();

    red_tiles
}

fn calculate_area(a: &Location, b: &Location) -> u64 {
    let dx = b.x.abs_diff(a.x) + 1;
    let dy = b.y.abs_diff(a.y) + 1;
    dx as u64 * dy as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let red_tiles = parse(input);
    Some(
        red_tiles
            .into_iter()
            .combinations(2)
            .map(|locations| calculate_area(locations.first().unwrap(), locations.last().unwrap()))
            .max()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let red_tiles = parse(input);
    Some(
        red_tiles
            .into_iter()
            .combinations(3)
            // TODO: problem not always within loop, do I need to calculate the actual loop?
            .filter_map(|locations| {
                let a = locations.first().unwrap();
                let b = locations.get(1).unwrap();
                let c = locations.last().unwrap();

                // a is corner of b and c
                if a.x == b.x && a.y == c.y || a.y == b.y && a.x == c.x {
                    return Some(calculate_area(b, c));
                }
                // b is corner of a and c
                if b.x == a.x && b.y == c.y || b.y == a.y && b.x == c.x {
                    return Some(calculate_area(a, c));
                }
                // c is corner of a and b
                if c.x == a.x && c.y == b.y || c.y == a.y && c.x == b.x {
                    return Some(calculate_area(a, b));
                }

                None
            })
            .max()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        assert_eq!(
            calculate_area(&Location { x: 2, y: 5 }, &Location { x: 9, y: 7 }),
            24
        );
        assert_eq!(
            calculate_area(&Location { x: 9, y: 7 }, &Location { x: 2, y: 5 }),
            24
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
