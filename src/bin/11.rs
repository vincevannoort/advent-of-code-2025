use itertools::Itertools;
use pathfinding::directed::count_paths::count_paths;
use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|line| {
            let (input_device, output_devices) = line.split_once(": ").unwrap();

            (
                input_device.to_string(),
                output_devices
                    .split_whitespace()
                    .map(|device| device.to_string())
                    .collect_vec(),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let devices = parse(input);
    let start: String = "you".to_string();
    let end: String = "out".to_string();
    let paths = count_paths(start, |p| devices.get(p).unwrap().clone(), |p| p == &end);
    Some(paths.try_into().unwrap())
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
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
