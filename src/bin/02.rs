use itertools::Itertools;

advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .split(",")
        .map(|line| {
            let (left, right) = line.split_once("-").unwrap();
            (
                left.trim().parse::<u64>().unwrap(),
                right.trim().parse::<u64>().unwrap(),
            )
        })
        .collect()
}

fn is_invalid_part_one(input: u64) -> bool {
    let input_string = input.to_string();
    if input_string.len().rem_euclid(2) == 1 {
        return false;
    }
    let (left, right) = input_string.split_at(input_string.len() / 2);
    left == right
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let result = input
        .iter()
        .flat_map(|(left, right)| (*left..=*right).collect_vec())
        .filter(|input: &u64| is_invalid_part_one(*input))
        .sum();

    Some(result)
}

fn is_invalid_part_two(input: u64) -> bool {
    let input_string = input.to_string();
    for i in 2..=input_string.len() {
        if input_string.len().rem_euclid(i) != 0 {
            continue;
        }
        let chunks = input_string
            .as_bytes()
            .chunks(input_string.len() / i)
            .collect_vec();
        if chunks.iter().all_equal() {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let result = input
        .iter()
        .flat_map(|(left, right)| (*left..=*right).collect::<Vec<u64>>())
        .filter(|input: &u64| is_invalid_part_two(*input))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_id() {
        assert!(is_invalid_part_one(446446));
        assert!(!is_invalid_part_one(145673));
        assert!(is_invalid_part_two(446446));
        assert!(is_invalid_part_two(565656));
        assert!(is_invalid_part_two(824824824));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
