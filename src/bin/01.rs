advent_of_code::solution!(1);

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    let mut position = 50;
    let mut zeroes = 0;

    for line in input.lines() {
        let Ok(distance) = line[1..].parse::<i32>() else {
            continue;
        };
        let rotation = match line.chars().next() {
            Some('L') => -distance,
            Some('R') => distance,
            _ => 0,
        };

        position = (position + rotation).rem_euclid(100);
        if position == 0 {
            zeroes += 1;
        }
    }

    Some(zeroes)
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    let mut position = 50;
    let mut zeroes = 0;

    for line in input.lines() {
        let Ok(distance) = line[1..].parse::<i32>() else {
            continue;
        };
        let step = match line.chars().next() {
            Some('L') => -1,
            _ => 1,
        };
        for _ in 0..distance {
            position = match position + step {
                -1 => 99,
                100 => 0,
                other => other,
            };
            if position == 0 {
                zeroes += 1;
            }
        }
    }

    Some(zeroes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
