use core::num;

use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug)]
struct Machine {
    lights: Vec<char>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
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
            let joltage = line
                .split_once(" {")
                .unwrap()
                .1
                .replace("}", "")
                .split(",")
                .map(|number| number.parse().unwrap())
                .collect_vec();

            Machine {
                lights,
                buttons,
                joltage,
            }
        })
        .collect();

    machines
}

fn toggle_buttons_matches_light_goal(light: &Vec<char>, buttons: &Vec<&Vec<usize>>) -> bool {
    let goal: Vec<bool> = light.iter().map(|c| *c == '#').collect_vec();
    let mut start: Vec<bool> = light.iter().map(|c| false).collect_vec();
    for button in buttons {
        for number in button.iter() {
            start[*number] = !start[*number]
        }
    }
    goal == start
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse(input);
    Some(
        machines
            .iter()
            .map(|machine| {
                let mut presses = 1;
                loop {
                    let combination =
                        machine
                            .buttons
                            .iter()
                            .combinations(presses)
                            .find(|buttons| {
                                toggle_buttons_matches_light_goal(&machine.lights, buttons)
                            });

                    if let Some(combination) = combination {
                        return combination.len() as u64;
                    } else {
                        presses += 1;
                    }
                }
            })
            .sum(),
    )
}

fn toggle_buttons_matches_joltage_goal(joltage: &Vec<usize>, buttons: &Vec<&Vec<usize>>) -> bool {
    // let goal: Vec<bool> = light.iter().map(|c| *c == '#').collect_vec();
    let mut start: Vec<usize> = joltage.iter().map(|_| 0).collect_vec();
    for button in buttons {
        for number in button.iter() {
            start[*number] += 1;
        }
    }
    *joltage == start
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse(input);
    Some(
        machines
            .iter()
            .map(|machine| {
                let mut presses = 1;
                loop {
                    let combination = machine
                        .buttons
                        .iter()
                        .combinations_with_replacement(presses)
                        // TODO: do something smart here, we know that we need a lot of presses
                        .find(|buttons| {
                            toggle_buttons_matches_joltage_goal(&machine.joltage, buttons)
                        });

                    if let Some(combination) = combination {
                        return combination.len() as u64;
                    } else {
                        presses += 1;
                    }
                }
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
