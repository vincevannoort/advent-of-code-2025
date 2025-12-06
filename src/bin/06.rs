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

fn parse(input: &str) -> Vec<(char, Vec<u64>)> {
    let mut lines = input.lines();

    let result: Vec<Vec<u64>> = transpose(
        lines
            .take_while_ref(|line| !line.contains("*"))
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
    let inputs = parse(input);
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

pub fn part_two(input: &str) -> Option<u64> {
    None
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
