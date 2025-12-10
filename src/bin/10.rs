use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug)]
struct Machine {
    lights: Vec<char>,
    buttons: Vec<Vec<u16>>,
}

fn parse(input: &str) -> Vec<Machine> {
    let machines: Vec<Machine> = input
        .lines()
        .map(|line| {
            let lights = line
                .split_once("] ")
                .unwrap()
                .0
                .chars()
                .skip(1)
                .collect_vec();
            let buttons = line
                .split_once("] ")
                .unwrap()
                .1
                .split_once(" {")
                .unwrap()
                .0
                .split_whitespace()
                .map(|button| {
                    button[1..button.len() - 1]
                        .split(",")
                        .map(|number| number.parse().unwrap())
                        .collect_vec()
                })
                .collect_vec();
            Machine { lights, buttons }
        })
        .collect();

    machines
}

pub fn part_one(input: &str) -> Option<u64> {
    let inputs = parse(input);
    dbg!(inputs);
    None
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
