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
    let (ranges, _) = parse(input);
    let sorted_ranges = ranges
        .into_iter()
        .sorted_by_key(|r| (*r.start(), *r.end()))
        .collect_vec();

    let merged_ranges: Vec<RangeInclusive<u64>> = sorted_ranges
        .into_iter()
        .coalesce(|left, right| {
            // fully contained
            if left.contains(right.start()) && left.contains(right.end()) {
                Ok(RangeInclusive::new(*left.start(), *left.end()))
            }
            // partly overlapping
            else if left.contains(right.start()) || left.contains(right.end()) {
                Ok(RangeInclusive::new(*left.start(), *right.end()))
            }
            // no overlap
            else {
                Err((left, right))
            }
        })
        .collect_vec();

    Some(
        merged_ranges
            .into_iter()
            .map(|range| std::convert::TryInto::<u64>::try_into(range.count()).unwrap())
            .sum(),
    )
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
    fn example1() {
        assert_eq!(part_two("3-5\n5-10\n\n"), Some(8));
    }

    #[test]
    fn example2() {
        assert_eq!(part_two("3-10\n5-7\n\n"), Some(8));
    }

    #[test]
    fn example3() {
        assert_eq!(part_two("3-8\n3-10\n\n"), Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
