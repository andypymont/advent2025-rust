advent_of_code::solution!(3);

fn max_joltage(line: &str, batteries: u32) -> Option<u64> {
    let (joltage, remainder) = line.split_at(batteries as usize);
    let mut joltage = joltage.parse::<u64>().ok()?;

    for digit in remainder
        .chars()
        .filter_map(|c| c.to_digit(10).map(u64::from))
    {
        joltage = (0..batteries)
            .map(|pos| {
                // split current joltage into two at the position index, leaving off
                // one digit from the left hand side
                let divisor = 10u64.pow(pos);
                let left = joltage.div_euclid(10 * divisor);
                let right = joltage.rem_euclid(divisor);

                // put them back together again with the right side one digit to the left,
                // and adding the new digit to the end
                (10 * ((left * divisor) + right)) + digit
            })
            .max()
            .map_or(joltage, |best| best.max(joltage));
    }

    Some(joltage)
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().filter_map(|line| max_joltage(line, 2)).sum())
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().filter_map(|line| max_joltage(line, 12)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_joltage() {
        assert_eq!(max_joltage("987654321111111", 2), Some(98));
        assert_eq!(max_joltage("811111111111119", 2), Some(89));
        assert_eq!(max_joltage("234234234234278", 2), Some(78));
        assert_eq!(max_joltage("818181911112111", 2), Some(92));
        assert_eq!(max_joltage("987654321111111", 12), Some(987654321111));
        assert_eq!(max_joltage("811111111111119", 12), Some(811111111119));
        assert_eq!(max_joltage("234234234234278", 12), Some(434234234278));
        assert_eq!(max_joltage("818181911112111", 12), Some(888911112111));
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
