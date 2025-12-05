use itertools::{Itertools, any};
use std::ops::RangeInclusive;

advent_of_code::solution!(5);

fn parse(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut lines = input.lines();
    let ranges = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (left, right) = l.split_once("-").unwrap();
            RangeInclusive::new(left.parse().unwrap(), right.parse().unwrap())
        })
        .collect_vec();

    let ids = lines.map(|l| l.parse().unwrap()).collect_vec();

    (ranges, ids)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = parse(input);
    Some(
        ids.iter()
            .filter(|id| any(ranges.clone(), |range| range.contains(id)))
            .count()
            .try_into()
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
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
