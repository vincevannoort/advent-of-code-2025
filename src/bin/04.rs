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

pub fn find_accessible_papers(grid: &Grid<char>) -> Vec<(&Location, &char)> {
    grid.locations
        .iter()
        .filter(|(_, c)| **c == '@')
        .filter(|(l, _)| {
            // TODO: this I think is really slow because we keep calculating it over and over and creating new vectors every time
            // it should really return references of locations instead
            let adjacent_location = grid.get_adjacent_locations(l);
            adjacent_location.iter().filter(|(_, c)| **c == '@').count() < 4
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse(input);
    let accessible_papers = find_accessible_papers(&grid);
    Some(accessible_papers.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse(input);
    let mut count = 0;

    loop {
        let accessible_papers = find_accessible_papers(&grid)
            .into_iter()
            .map(|(l, _)| *l)
            .collect_vec();

        if accessible_papers.is_empty() {
            break;
        }

        count += accessible_papers.len();

        for accessible_paper in accessible_papers {
            grid.locations.insert(accessible_paper, 'x');
        }
    }

    Some(count.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
