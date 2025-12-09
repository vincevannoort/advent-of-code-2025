use advent_of_code::{Grid, Location};
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
    None
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
        assert_eq!(result, None);
    }
}
