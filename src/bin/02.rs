advent_of_code::solution!(2);

fn is_invalid(number: u64, check_all: bool) -> bool {
    let number_str = number.to_string();
    let mid = number_str.len() / 2;

    if check_all {
        (1..=mid).any(|length| {
            let times = number_str.len() / length;
            number_str[..length].repeat(times) == number_str
        })
    } else {
        number_str[..mid] == number_str[mid..]
    }
}

fn total_invalid_in_range(range: &str, check_all: bool) -> u64 {
    let Some((start, finish)) = range.split_once('-') else {
        return 0;
    };
    let Ok(start) = start.parse() else {
        return 0;
    };
    let Ok(finish) = finish.parse() else {
        return 0;
    };
    (start..=finish).filter(|x| is_invalid(*x, check_all)).sum()
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .split(',')
            .map(|range| total_invalid_in_range(range, false))
            .sum(),
    )
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .split(',')
            .map(|range| total_invalid_in_range(range, true))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_invalid_in_range() {
        assert_eq!(total_invalid_in_range("11-22", false), 33);
        assert_eq!(total_invalid_in_range("95-115", false), 99);
        assert_eq!(total_invalid_in_range("222220-222224", false), 222222);
        assert_eq!(total_invalid_in_range("16985222-1698528", false), 0);
        assert_eq!(total_invalid_in_range("824824821-824824827", false), 0);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1_227_775_554));
    }

    #[test]
    fn test_total_invalid_in_range_check_all() {
        assert_eq!(total_invalid_in_range("11-22", true), 33);
        assert_eq!(total_invalid_in_range("99-115", true), 210);
        assert_eq!(total_invalid_in_range("222220-222224", true), 222222);
        assert_eq!(total_invalid_in_range("16985222-1698528", true), 0);
        assert_eq!(
            total_invalid_in_range("824824821-824824827", true),
            824824824
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4_174_379_265));
    }
}
