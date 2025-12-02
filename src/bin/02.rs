advent_of_code::solution!(2);

fn is_invalid(number: u64) -> bool {
    let number_str = number.to_string();
    let mid = number_str.len() / 2;
    number_str[..mid] == number_str[mid..]
}

fn total_invalid_in_range(range: &str) -> u64 {
    let Some((start, finish)) = range.split_once('-') else {
        return 0;
    };
    let Ok(start) = start.parse() else {
        return 0;
    };
    let Ok(finish) = finish.parse() else {
        return 0;
    };
    (start..=finish).filter(|x| is_invalid(*x)).sum()
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    Some(input.trim().split(',').map(total_invalid_in_range).sum())
}

#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_in_range() {
        assert_eq!(total_invalid_in_range("11-22"), 33);
        assert_eq!(total_invalid_in_range("95-115"), 99);
        assert_eq!(total_invalid_in_range("222220-222224"), 222222);
        assert_eq!(total_invalid_in_range("16985222-1698528"), 0);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1_227_775_554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
