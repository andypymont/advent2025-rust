use std::collections::BTreeSet;
use std::str::FromStr;

advent_of_code::solution!(10);

#[derive(Debug, PartialEq)]
struct Machine {
    indicators: u64,
    pattern: u64,
    buttons: Vec<u64>,
    requirements: Vec<u64>,
}

impl Machine {
    fn fewest_button_presses(&self) -> u64 {
        let mut queue = BTreeSet::new();
        queue.insert((0, 0));

        while let Some((presses, lights)) = queue.pop_first() {
            if lights == self.pattern {
                return presses;
            }

            queue.extend(
                self.buttons
                    .iter()
                    .map(|button| (presses + 1, lights ^ button)),
            );
        }

        u64::MAX
    }
}

#[derive(Debug, PartialEq)]
struct Factory {
    machines: Vec<Machine>,
}

impl Factory {
    fn fewest_button_presses(&self) -> u64 {
        self.machines
            .iter()
            .map(Machine::fewest_button_presses)
            .sum()
    }
}

#[derive(Debug, PartialEq)]
struct ParseFactoryError;

impl FromStr for Machine {
    type Err = ParseFactoryError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (indicators, line) = line.split_once(' ').ok_or(ParseFactoryError)?;
        let (buttons, requirements) = line.rsplit_once(' ').ok_or(ParseFactoryError)?;

        let (indicators, pattern) = indicators
            .chars()
            .filter_map(|ch| match ch {
                '.' => Some(0),
                '#' => Some(1),
                _ => None,
            })
            .fold((0, 0), |(indicators, pattern), on| {
                (indicators + 1, pattern << 1 | on)
            });

        Ok(Self {
            indicators,
            pattern,
            buttons: buttons
                .split_whitespace()
                .filter_map(|button| {
                    button
                        .strip_prefix('(')
                        .and_then(|button| button.strip_suffix(')'))
                        .map(|button| {
                            button
                                .split(',')
                                .filter_map(|light| light.parse().ok())
                                .fold(0, |lights, light: u64| {
                                    lights + (1 << (indicators - light - 1))
                                })
                        })
                })
                .collect(),
            requirements: requirements
                .strip_prefix("{")
                .and_then(|reqs| reqs.strip_suffix("}"))
                .unwrap_or("")
                .split(',')
                .filter_map(|light| light.parse().ok())
                .collect(),
        })
    }
}

impl FromStr for Factory {
    type Err = ParseFactoryError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut machines = Vec::new();
        for line in input.lines() {
            let machine = line.parse()?;
            machines.push(machine);
        }
        Ok(Self { machines })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    Factory::from_str(input)
        .ok()
        .as_ref()
        .map(Factory::fewest_button_presses)
}

#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_factory() -> Factory {
        Factory {
            machines: vec![
                Machine {
                    indicators: 4,
                    pattern: 6,
                    buttons: vec![1, 5, 2, 3, 10, 12],
                    requirements: vec![3, 5, 4, 7],
                },
                Machine {
                    indicators: 5,
                    pattern: 2,
                    buttons: vec![23, 6, 17, 28, 15],
                    requirements: vec![7, 5, 12, 7, 2],
                },
                Machine {
                    indicators: 6,
                    pattern: 29,
                    buttons: vec![62, 38, 59, 24],
                    requirements: vec![10, 11, 11, 5, 10, 5],
                },
            ],
        }
    }

    #[test]
    fn test_parse_factory() {
        assert_eq!(
            Factory::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_factory()),
        );
    }

    #[test]
    fn test_fewest_button_presses() {
        let factory = example_factory();
        assert_eq!(factory.machines[0].fewest_button_presses(), 2);
        assert_eq!(factory.machines[1].fewest_button_presses(), 3);
        assert_eq!(factory.machines[2].fewest_button_presses(), 2);
    }

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
