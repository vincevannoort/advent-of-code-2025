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
    if input_string.len() % 2 == 1 {
        return false;
    }
    let (left, right) = input_string.split_at(input_string.len() / 2);
    left == right
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let result = input
        .iter()
        .flat_map(|(left, right)| (*left..=*right).collect::<Vec<u64>>())
        .filter(|input: &u64| is_invalid_part_one(*input))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_id() {
        assert!(is_invalid_part_one(446446));
        assert!(!is_invalid_part_one(145673));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
