use std::cmp::max;

use itertools::Itertools;

advent_of_code::solution!(3);

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let inputs = parse(input);
    let mut total = 0;
    for input in inputs {
        let mut best = u64::MIN;
        for (index, num) in input.iter().enumerate() {
            let remaining_nums = &input[(index + 1)..];
            for remaining_num in remaining_nums {
                best = max(best, (num * 10 + remaining_num) as u64);
            }
        }
        total += best;
    }
    Some(total)
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
