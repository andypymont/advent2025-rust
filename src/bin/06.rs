advent_of_code::solution!(6);

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    let mut totals = Vec::new();
    let mut is_multiplier = Vec::new();

    let mut lines = input.lines().rev();

    // first line from the bottom contains the operators
    let line = lines.next()?;
    for operator in line.split_whitespace() {
        if operator == "*" {
            totals.push(1);
            is_multiplier.push(true);
        } else {
            totals.push(0);
            is_multiplier.push(false);
        }
    }

    // remaining lines then need to be added or multiplied to total
    for line in lines {
        for (ix, value) in line.split_whitespace().enumerate() {
            if ix >= totals.len() {
                return None;
            }
            let value: u64 = value.parse().ok()?;
            if is_multiplier[ix] {
                totals[ix] *= value;
            } else {
                totals[ix] += value;
            }
        }
    }

    Some(totals.iter().sum())
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
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4_277_556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
