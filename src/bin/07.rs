use core::panic;
use std::collections::{HashMap, HashSet};

use advent_of_code::{Direction, Grid, Location};
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

fn get_beams(grid: &Grid<char>) -> Vec<(Location, char)> {
    grid.locations
        .iter()
        .filter(|(_, c)| **c == '|')
        .map(|(l, c)| (*l, *c))
        .collect_vec()
}

fn count_beams(grid: &Grid<char>) -> usize {
    grid.locations.iter().filter(|(_, c)| **c == '|').count()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = parse(input);
    let mut splits: HashSet<Location> = HashSet::new();

    loop {
        let beams_before = count_beams(&grid);
        let beams = get_beams(&grid);

        for (l, _) in beams {
            if let Some((down, c)) = grid.get_by_direction(&l, Direction::Down) {
                match c {
                    '.' => grid.locations.insert(down, '|'),
                    '^' => {
                        splits.insert(down);
                        if let Some((left, _)) = grid.get_by_direction(&down, Direction::Left) {
                            grid.locations.insert(left, '|');
                        };
                        if let Some((right, _)) = grid.get_by_direction(&down, Direction::Right) {
                            grid.locations.insert(right, '|');
                        };
                        None
                    }
                    '|' => None,
                    _ => panic!("unknown"),
                };
            }
        }

        let beams_after = count_beams(&grid);
        if beams_before == beams_after {
            break;
        }
    }
    Some(splits.len().try_into().unwrap())
}

fn update_count(beams: &mut HashMap<Location, u64>, location: Location, count: u64) {
    beams
        .entry(location)
        .and_modify(|counter| *counter += count)
        .or_insert(count);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse(input);
    let mut options: u64 = 0;
    let mut beams: HashMap<Location, u64> = HashMap::new();
    beams.insert(
        *grid.locations.iter().find(|(_, c)| **c == '|').unwrap().0,
        1,
    );

    loop {
        let mut next_beams: HashMap<Location, u64> = HashMap::new();

        for (beam, count) in beams.clone() {
            if let Some((down, c)) = grid.get_by_direction(&beam, Direction::Down) {
                match c {
                    '.' => update_count(&mut next_beams, down, count),
                    '^' => {
                        if let Some((left, _)) = grid.get_by_direction(&down, Direction::Left) {
                            update_count(&mut next_beams, left, count);
                        };
                        if let Some((right, _)) = grid.get_by_direction(&down, Direction::Right) {
                            update_count(&mut next_beams, right, count);
                        };
                    }
                    '|' => (),
                    _ => panic!("unknown"),
                };
            } else {
                options += count;
            }
        }

        for (beam, count) in next_beams.clone() {
            grid.locations
                .insert(beam, count.to_string().chars().last().unwrap());
        }

        if next_beams.is_empty() {
            break;
        }

        beams = next_beams;
    }
    Some(options)
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
    fn test_six_layers() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_eight_layers() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 8,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_ten_layers() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 10,
        ));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
