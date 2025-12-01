advent_of_code::solution!(1);

fn parse_input(input: &str) -> Vec<(char, i32)> {
    input
        .lines()
        .map(|line| {
            (
                line.chars().next().unwrap(),
                line[1..].parse::<i32>().unwrap(),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial: i32 = 50;
    let mut count: u64 = 0;
    let inputs = parse_input(input);
    for (direction, amount) in inputs {
        dial = match direction {
            'L' => (dial - amount).rem_euclid(100),
            'R' => (dial + amount).rem_euclid(100),
            _ => panic!("unknown direction"),
        };
        if dial == 0 {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial: i32 = 50;
    let mut count: u64 = 0;
    let inputs = parse_input(input);
    for (direction, amount) in inputs {
        for _ in 0..amount {
            dial = match direction {
                'L' => (dial - 1).rem_euclid(100),
                'R' => (dial + 1).rem_euclid(100),
                _ => panic!("unknown direction"),
            };
            if dial == 0 {
                count += 1;
            }
        }
    }
    Some(count)
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
        assert_eq!(result, Some(6));
    }
}
