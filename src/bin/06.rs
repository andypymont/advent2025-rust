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

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<Option<u64>>> = Vec::new();

    for line in input.lines() {
        let check_char = line.chars().next()?;
        if check_char == '+' || check_char == '*' {
            // we've reached the line with the operators so we use their positions to put things
            // together
            return Some(
                line.char_indices()
                    .rev()
                    .fold((0, line.len()), |(total, prev_ix), (ix, op)| {
                        let numbers = (ix..prev_ix)
                            .map(|col| {
                                grid.iter()
                                    .filter_map(|row| *row.get(col).unwrap_or(&None))
                                    .fold(0, |acc, digit| (acc * 10) + digit)
                            })
                            .filter(|v| v != &0);
                        match op {
                            '*' => (total + numbers.product::<u64>(), ix),
                            '+' => (total + numbers.sum::<u64>(), ix),
                            _ => (total, prev_ix),
                        }
                    })
                    .0,
            );
        }

        // otherwise, the line contains numbers so we store them in a grid to use later
        let mut row = Vec::new();
        for ch in line.chars() {
            let digit = ch.to_digit(10).map(u64::from);
            row.push(digit);
        }
        grid.push(row);
    }

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
        assert_eq!(result, Some(3_263_827));
    }
}
