use core::panic;
use std::collections::{HashMap, HashSet};

use advent_of_code::{Grid, Location};
use itertools::Itertools;

advent_of_code::solution!(7);

fn parse(input: &str) -> Grid<char> {
    let obstacles: HashMap<Location, char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, c)| {
                let location = Location {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                };
                Some((location, if c == 'S' { '|' } else { c }))
            })
        })
        .collect();

    Grid {
        locations: obstacles,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = parse(input);
    let mut splits: HashSet<Location> = HashSet::new();
    loop {
        let beams_before = grid.locations.iter().filter(|(_, c)| **c == '|').count();
        let beams = grid
            .locations
            .iter()
            .filter(|(_, c)| **c == '|')
            // TODO: can I do this differently
            .map(|(l, c)| (*l, *c))
            .collect_vec();

        for (l, _) in beams {
            if let Some((down, c)) = grid.get_by_direction(&l, advent_of_code::Direction::Down) {
                match c {
                    '.' => grid.locations.insert(down, '|'),
                    '^' => {
                        splits.insert(down);
                        if let Some((left, _)) =
                            grid.get_by_direction(&down, advent_of_code::Direction::Left)
                        {
                            grid.locations.insert(left, '|');
                        };
                        if let Some((right, _)) =
                            grid.get_by_direction(&down, advent_of_code::Direction::Right)
                        {
                            grid.locations.insert(right, '|');
                        };
                        None
                    }
                    '|' => None,
                    _ => panic!("unknown"),
                };
            }
        }

        let beams_after = grid.locations.iter().filter(|(_, c)| **c == '|').count();

        if beams_before == beams_after {
            break;
        }
    }
    Some(splits.len().try_into().unwrap())
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
