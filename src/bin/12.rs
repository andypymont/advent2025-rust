use std::str::FromStr;

advent_of_code::solution!(12);

const SHAPE_COUNT: usize = 6;

#[derive(Debug, PartialEq)]
struct Puzzle {
    area: usize,
    quantities: [usize; SHAPE_COUNT],
}

#[derive(Debug, PartialEq)]
struct Solutions {
    clear_fit: usize,
    clear_no_fit: usize,
    unclear: usize,
}

#[derive(Debug, PartialEq)]
struct Situation {
    shape_sizes: [usize; SHAPE_COUNT],
    puzzles: Vec<Puzzle>,
}

impl Situation {
    fn find_solutions(&self) -> Solutions {
        let (clear_fit, clear_no_fit, unclear) =
            self.puzzles
                .iter()
                .fold((0, 0, 0), |(clear_fit, clear_no_fit, unclear), puzzle| {
                    let required: usize = puzzle
                        .quantities
                        .iter()
                        .enumerate()
                        .map(|(ix, quantity)| self.shape_sizes[ix] * quantity)
                        .sum();

                    if required > puzzle.area {
                        return (clear_fit, clear_no_fit + 1, unclear);
                    }

                    if (required * 4 / 3) < puzzle.area {
                        return (clear_fit + 1, clear_no_fit, unclear);
                    }

                    (clear_fit, clear_no_fit, unclear + 1)
                });

        Solutions {
            clear_fit,
            clear_no_fit,
            unclear,
        }
    }
}

#[derive(Debug, PartialEq)]
struct ParseSituationError;

impl FromStr for Puzzle {
    type Err = ParseSituationError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (area, quantities) = line.split_once(": ").ok_or(ParseSituationError)?;
        let (width, height) = area.split_once('x').ok_or(ParseSituationError)?;
        let width: usize = width.parse().map_err(|_| ParseSituationError)?;
        let height: usize = height.parse().map_err(|_| ParseSituationError)?;

        let quantities = {
            let mut parsed = [0; SHAPE_COUNT];
            for (ix, quantity) in quantities.split_whitespace().enumerate() {
                let quantity = quantity.parse().map_err(|_| ParseSituationError)?;
                parsed[ix] = quantity;
            }
            parsed
        };

        Ok(Self {
            area: width * height,
            quantities,
        })
    }
}

impl FromStr for Situation {
    type Err = ParseSituationError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut shape_sizes = [0; SHAPE_COUNT];
        let mut puzzles = Vec::new();

        for section in input.split("\n\n") {
            let mut lines = section.lines().peekable();
            if lines.peek().is_some_and(|line| line.ends_with(':')) {
                // shape
                let ix = lines
                    .next()
                    .and_then(|line| line.strip_suffix(':'))
                    .and_then(|line| line.parse::<usize>().ok())
                    .ok_or(ParseSituationError)?;
                let size = lines
                    .map(|x| x.chars().filter(|ch| *ch == '#').count())
                    .sum();
                shape_sizes[ix] = size;

                continue;
            }

            // list of puzzles
            for line in lines {
                let puzzle = Puzzle::from_str(line)?;
                puzzles.push(puzzle);
            }
        }

        Ok(Self {
            shape_sizes,
            puzzles,
        })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Situation::from_str(input)
        .ok()
        .as_ref()
        .map(Situation::find_solutions)
        .and_then(|solutions| {
            if solutions.unclear > 0 {
                None
            } else {
                Some(solutions.clear_fit)
            }
        })
}

#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_situation() -> Situation {
        Situation {
            shape_sizes: [7, 7, 7, 7, 7, 7],
            puzzles: vec![
                Puzzle {
                    area: 16,
                    quantities: [0, 0, 0, 0, 2, 0],
                },
                Puzzle {
                    area: 60,
                    quantities: [1, 0, 1, 0, 2, 2],
                },
                Puzzle {
                    area: 60,
                    quantities: [1, 0, 1, 0, 3, 2],
                },
            ],
        }
    }

    #[test]
    fn test_parse_situation() {
        assert_eq!(
            Situation::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_situation()),
        );
    }

    #[test]
    fn test_find_solutions() {
        assert_eq!(
            example_situation().find_solutions(),
            Solutions {
                clear_fit: 1,
                clear_no_fit: 0,
                unclear: 2,
            },
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
