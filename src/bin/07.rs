advent_of_code::solution!(7);

const WIDTH: usize = 141;

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    let mut beams = [false; WIDTH];
    let mut splits = 0;

    for line in input.lines() {
        let mut next = [false; WIDTH];

        for (col, ch) in line.char_indices() {
            match ch {
                'S' => next[col] = true,
                '^' => {
                    if beams[col] {
                        next[col - 1] = true;
                        next[col + 1] = true;
                        splits += 1;
                    }
                }
                _ => {
                    if beams[col] {
                        next[col] = true;
                    }
                }
            }
        }
        beams = next;
    }

    Some(splits)
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    let mut timelines = [0; WIDTH];

    for line in input.lines() {
        let mut next = [0; WIDTH];

        for (col, ch) in line.char_indices() {
            match ch {
                'S' => next[col] = 1,
                '^' => {
                    next[col - 1] += timelines[col];
                    next[col + 1] += timelines[col];
                }
                _ => next[col] += timelines[col],
            }
        }
        timelines = next;
    }

    Some(timelines.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
