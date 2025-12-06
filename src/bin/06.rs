advent_of_code::solution!(6);

use itertools::Itertools;

// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn parse_part_one(input: &str) -> Vec<(char, Vec<u64>)> {
    let mut lines = input.lines();

    let result: Vec<Vec<u64>> = transpose(
        lines
            .take_while_ref(|line| !line.contains("*") && !line.contains("+"))
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect()
            })
            .collect_vec(),
    );

    let operations = lines
        .last()
        .unwrap()
        .chars()
        .filter(|c| *c == '*' || *c == '+')
        .collect_vec();

    operations.into_iter().zip_eq(result).collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let inputs = parse_part_one(input);
    Some(
        inputs
            .into_iter()
            .map(|(operation, numbers)| {
                numbers
                    .into_iter()
                    .reduce(|acc, number| match operation {
                        '*' => acc * number,
                        '+' => acc + number,
                        _ => panic!("test"),
                    })
                    .unwrap()
            })
            .sum(),
    )
}

fn parse_part_two(input: &str) -> Vec<(char, Vec<u64>)> {
    // transpose all numbers
    let inputs = transpose(
        input
            .lines()
            .take_while_ref(|line| !line.contains("*") && !line.contains("+"))
            .map(|line| line.chars().collect_vec())
            .collect_vec(),
    );

    // split based on empty row (the divider between groups), then convert to number
    let numbers: Vec<Vec<u64>> = inputs
        .iter()
        .map(|number| number.iter().collect::<String>())
        .collect_vec()
        .split(|number| number.trim().is_empty())
        .map(|s| s.to_vec())
        .map(|numbers| {
            numbers
                .iter()
                .map(|number| number.trim().parse().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let operations = input
        .lines()
        .last()
        .unwrap()
        .chars()
        .filter(|c| *c == '*' || *c == '+')
        .collect_vec();

    operations.into_iter().zip_eq(numbers).collect_vec()
}

pub fn part_two(input: &str) -> Option<u64> {
    let inputs = parse_part_two(input);
    Some(
        inputs
            .into_iter()
            .map(|(operation, numbers)| {
                numbers
                    .into_iter()
                    .reduce(|acc, number| match operation {
                        '*' => acc * number,
                        '+' => acc + number,
                        _ => panic!("test"),
                    })
                    .unwrap()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_example_1() {
        assert_eq!(part_two("64 \n23 \n314\n+  "), Some(1058));
    }

    #[test]
    fn test_example_2() {
        assert_eq!(part_two(" 51\n387\n215\n*  "), Some(3253600));
    }

    #[test]
    fn test_example_3() {
        assert_eq!(part_two("328\n64 \n98 \n+  "), Some(625));
    }

    #[test]
    fn test_example_4() {
        assert_eq!(
            part_two(" 51 64 \n387 23 \n215 314\n*   +  "),
            Some(1058 + 3253600)
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
