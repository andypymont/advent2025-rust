use microlp::{ComparisonOp, LinearExpr, OptimizationDirection, Problem, Variable};
use std::collections::BTreeSet;
use std::str::FromStr;

advent_of_code::solution!(10);

#[derive(Debug, PartialEq)]
struct Machine {
    pattern: u64,
    buttons: Vec<u64>,
    joltages: Vec<u64>,
}

impl Machine {
    fn fewest_button_presses(&self) -> Option<u64> {
        let mut queue = BTreeSet::new();
        queue.insert((0, 0));

        while let Some((presses, lights)) = queue.pop_first() {
            if lights == self.pattern {
                return Some(presses);
            }

            queue.extend(
                self.buttons
                    .iter()
                    .map(|button| (presses + 1, lights ^ button)),
            );
        }

        None
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_sign_loss)]
    fn fewest_button_presses_joltage(&self) -> Option<u64> {
        let mut problem = Problem::new(OptimizationDirection::Minimize);

        let vars: Vec<Variable> = self
            .buttons
            .iter()
            .map(|_| problem.add_integer_var(1.0, (0, i32::MAX)))
            .collect();

        for (constraint, joltage) in self.joltages.iter().enumerate() {
            let mut equation = LinearExpr::empty();

            for (variable, button) in self.buttons.iter().enumerate() {
                if button & (1 << constraint) != 0 {
                    equation.add(vars[variable], 1.0);
                }
            }

            problem.add_constraint(equation, ComparisonOp::Eq, *joltage as f64);
        }

        problem
            .solve()
            .ok()
            .map(|solution| solution.objective().round() as u64)
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
            .filter_map(Machine::fewest_button_presses)
            .sum()
    }

    fn fewest_button_presses_joltage(&self) -> u64 {
        self.machines
            .iter()
            .filter_map(Machine::fewest_button_presses_joltage)
            .sum()
    }
}

#[derive(Debug, PartialEq)]
struct ParseFactoryError;

impl FromStr for Machine {
    type Err = ParseFactoryError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (indicators, line) = line.split_once(' ').ok_or(ParseFactoryError)?;
        let (buttons, joltages) = line.rsplit_once(' ').ok_or(ParseFactoryError)?;
        let indicators = indicators
            .strip_prefix("[")
            .and_then(|ind| ind.strip_suffix("]"))
            .ok_or(ParseFactoryError)?;

        Ok(Self {
            pattern: indicators
                .chars()
                .rev()
                .map(|ch| u64::from(ch == '#'))
                .fold(0, |pattern, on| pattern << 1 | on),
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
                                .fold(0, |lights, light: u64| lights | (1 << light))
                        })
                })
                .collect(),
            joltages: joltages
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

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    Factory::from_str(input)
        .ok()
        .as_ref()
        .map(Factory::fewest_button_presses_joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_factory() -> Factory {
        Factory {
            machines: vec![
                Machine {
                    pattern: 6,
                    buttons: vec![8, 10, 4, 12, 5, 3],
                    joltages: vec![3, 5, 4, 7],
                },
                Machine {
                    pattern: 8,
                    buttons: vec![29, 12, 17, 7, 30],
                    joltages: vec![7, 5, 12, 7, 2],
                },
                Machine {
                    pattern: 46,
                    buttons: vec![31, 25, 55, 6],
                    joltages: vec![10, 11, 11, 5, 10, 5],
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
        assert_eq!(factory.machines[0].fewest_button_presses(), Some(2));
        assert_eq!(factory.machines[1].fewest_button_presses(), Some(3));
        assert_eq!(factory.machines[2].fewest_button_presses(), Some(2));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_fewest_button_presses_joltage() {
        let factory = example_factory();
        assert_eq!(
            factory.machines[0].fewest_button_presses_joltage(),
            Some(10)
        );
        assert_eq!(
            factory.machines[1].fewest_button_presses_joltage(),
            Some(12)
        );
        assert_eq!(
            factory.machines[2].fewest_button_presses_joltage(),
            Some(11)
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
