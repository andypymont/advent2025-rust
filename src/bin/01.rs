advent_of_code::solution!(1);

fn rotations(input: &str) -> impl Iterator<Item = i32> {
    input.lines().filter_map(|line| {
        let distance: i32 = line[1..].parse().ok()?;
        line.chars().next().map(|ch| match ch {
            'L' => -distance,
            'R' => distance,
            _ => 0,
        })
    })
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    Some(
        rotations(input)
            .fold((50, 0), |(position, zeroes), rotation| {
                let position = (position + rotation).rem_euclid(100);
                (position, zeroes + u64::from(position == 0))
            })
            .1,
    )
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    Some(
        rotations(input)
            .flat_map(|rotations| {
                let step = rotations.signum();
                let distance = rotations / step;
                (0..distance).map(move |_| step)
            })
            .fold((50, 0), |(position, zeroes), step| {
                let position = match position + step {
                    -1 => 99,
                    100 => 0,
                    other => other,
                };
                (position, zeroes + u64::from(position == 0))
            })
            .1,
    )
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
