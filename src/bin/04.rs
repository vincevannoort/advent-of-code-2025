use std::collections::HashMap;

use advent_of_code::{Grid, Location};
use itertools::Itertools;

advent_of_code::solution!(4);

fn parse(input: &str) -> Grid<char> {
    let obstacles: HashMap<Location, char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, char)| {
                let location = Location {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                };
                Some((location, char))
            })
        })
        .collect();

    Grid {
        locations: obstacles,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse(input);
    dbg!(grid.get_adjacent_locations(&Location { x: 1, y: 1 }));

    let accessible_papers = grid
        .locations
        .iter()
        .filter(|(_, c)| **c == '@')
        .filter(|(l, _)| {
            let adjacent_location = grid.get_adjacent_locations(l);
            adjacent_location.iter().filter(|(_, c)| **c == '@').count() < 4
        })
        .collect_vec();

    Some(accessible_papers.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
