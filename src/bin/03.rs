advent_of_code::solution!(3);

fn max_joltage(line: &str) -> Option<u32> {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));

    let mut a = digits.next()?;
    let mut b = digits.next()?;

    for digit in digits {
        if digit > b {
            if b > a {
                a = b;
            }
            b = digit;
        } else if b > a {
            a = b;
            b = digit;
        }
    }

    Some((a * 10) + b)
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().filter_map(max_joltage).sum())
}

#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_joltage() {
        assert_eq!(max_joltage("987654321111111"), Some(98));
        assert_eq!(max_joltage("811111111111119"), Some(89));
        assert_eq!(max_joltage("234234234234278"), Some(78));
        assert_eq!(max_joltage("818181911112111"), Some(92));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
