advent_of_code::solution!(3);

fn max_joltage_pair(line: &str) -> Option<u64> {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10).map(u64::from));
    let mut a = digits.next()?;
    let mut b = digits.next()?;

    for digit in digits {
        if b > a {
            a = b;
            b = digit;
            continue;
        }
        if digit > b {
            b = digit;
        }
    }

    Some((a * 10) + b)
}

const BATTERIES_SIZE: usize = 12;

#[derive(Clone, Debug, PartialEq)]
struct Batteries {
    batteries: [u64; BATTERIES_SIZE],
    joltage: u64,
}

impl Batteries {
    fn calculate_joltage(digits: &[u64]) -> u64 {
        digits.iter().fold(0, |total, digit| (total * 10) + digit)
    }

    fn candidate(&self, pos: usize, new_digit: u64) -> Self {
        let mut batteries = self.batteries;
        batteries[pos..(BATTERIES_SIZE - 1)]
            .copy_from_slice(&self.batteries[(pos + 1)..BATTERIES_SIZE]);
        batteries[BATTERIES_SIZE - 1] = new_digit;

        Self {
            batteries,
            joltage: Self::calculate_joltage(&batteries),
        }
    }

    fn push(&mut self, digit: u64) {
        let best = (0..BATTERIES_SIZE)
            .map(|pos| self.candidate(pos, digit))
            .fold(self.clone(), |best, candidate| {
                if candidate.joltage > best.joltage {
                    candidate
                } else {
                    best
                }
            });
        self.batteries = best.batteries;
        self.joltage = best.joltage;
    }
}

#[derive(Debug, PartialEq)]
struct ParseBatteriesError;

impl TryFrom<&str> for Batteries {
    type Error = ParseBatteriesError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut batteries = [0; BATTERIES_SIZE];
        let mut digits = line.chars().filter_map(|c| c.to_digit(10).map(u64::from));

        for battery in batteries.iter_mut().take(BATTERIES_SIZE) {
            let digit = digits.next().ok_or(ParseBatteriesError)?;
            *battery = digit;
        }

        Ok(Self {
            batteries,
            joltage: Self::calculate_joltage(&batteries),
        })
    }
}

fn max_joltage(line: &str) -> Option<u64> {
    let mut batteries = Batteries::try_from(line).ok()?;

    for digit in line
        .chars()
        .filter_map(|c| c.to_digit(10).map(u64::from))
        .skip(BATTERIES_SIZE)
    {
        batteries.push(digit);
    }

    Some(batteries.joltage)
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().filter_map(max_joltage_pair).sum())
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().filter_map(max_joltage).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_joltage_pair() {
        assert_eq!(max_joltage_pair("987654321111111"), Some(98));
        assert_eq!(max_joltage_pair("811111111111119"), Some(89));
        assert_eq!(max_joltage_pair("234234234234278"), Some(78));
        assert_eq!(max_joltage_pair("818181911112111"), Some(92));
    }

    #[test]
    fn test_max_joltage() {
        assert_eq!(max_joltage("987654321111111"), Some(987654321111));
        assert_eq!(max_joltage("811111111111119"), Some(811111111119));
        assert_eq!(max_joltage("234234234234278"), Some(434234234278));
        assert_eq!(max_joltage("818181911112111"), Some(888911112111));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
