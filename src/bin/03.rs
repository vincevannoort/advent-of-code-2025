use std::{cmp::max as max_cmp, option};

use itertools::{Itertools, max as max_iter};

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
    let mut result = 0;
    for input in inputs {
        let mut best = u64::MIN;
        for (index, num) in input.iter().enumerate() {
            let remaining_nums = &input[(index + 1)..];
            for remaining_num in remaining_nums {
                best = max_cmp(best, (num * 10 + remaining_num) as u64);
            }
        }
        result += best;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let inputs = parse(input);
    let mut result = 0;
    for input in inputs {
        let mut index = 0;
        let mut inner_result = String::from("");
        for num in (0..12).rev() {
            let options = &input[index..(input.len() - num)];
            let best_option = options.iter().max().unwrap();
            let (best_index, _) = options.iter().find_position(|x| *x == best_option).unwrap();
            inner_result.push_str(&best_option.to_string());
            index = index + best_index + 1;
        }
        result += inner_result.parse::<u64>().unwrap()
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(part_two("2311111111111"), Some(311111111111));
    }

    #[test]
    fn test_example2() {
        assert_eq!(part_two("987654321111111"), Some(987654321111));
    }

    #[test]
    fn test_example3() {
        assert_eq!(part_two("811111111111119"), Some(811111111119));
    }

    #[test]
    fn test_example4() {
        assert_eq!(part_two("234234234234278"), Some(434234234278));
    }

    #[test]
    fn test_example5() {
        assert_eq!(part_two("818181911112111"), Some(888911112111));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
