use std::str::FromStr;

advent_of_code::solution!(5);

#[derive(Debug, PartialEq)]
struct Kitchen {
    fresh_ranges: Vec<(u64, u64)>,
    ingredients: Vec<u64>,
}

impl Kitchen {
    fn fresh_ingredients(&self) -> u64 {
        self.ingredients
            .iter()
            .map(|ingredient| u64::from(self.is_fresh(*ingredient)))
            .sum()
    }

    fn total_fresh_ingredients(&self) -> u64 {
        self.fresh_ranges
            .iter()
            .fold((0, 0), |(max, count), (start, finish)| {
                if *start > max {
                    return (*finish, count + 1 + finish - start);
                }
                if *finish > max {
                    return (*finish, count + finish - max);
                }
                (max, count)
            })
            .1
    }

    fn is_fresh(&self, ingredient: u64) -> bool {
        self.fresh_ranges
            .iter()
            .any(|(start, finish)| ingredient >= *start && ingredient <= *finish)
    }
}

#[derive(Debug, PartialEq)]
struct ParseKitchenError;

impl FromStr for Kitchen {
    type Err = ParseKitchenError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let Some((fresh_ranges_str, ingredients_str)) = input.split_once("\n\n") else {
            return Err(ParseKitchenError);
        };

        let mut fresh_ranges = Vec::new();
        for line in fresh_ranges_str.lines() {
            let Some((start, finish)) = line.split_once('-') else {
                return Err(ParseKitchenError);
            };
            let start = start.parse().map_err(|_| ParseKitchenError)?;
            let finish = finish.parse().map_err(|_| ParseKitchenError)?;
            fresh_ranges.push((start, finish));
        }
        fresh_ranges.sort_unstable();

        let mut ingredients = Vec::new();
        for line in ingredients_str.lines() {
            let ingredient = line.parse().map_err(|_| ParseKitchenError)?;
            ingredients.push(ingredient);
        }

        Ok(Self {
            fresh_ranges,
            ingredients,
        })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    Kitchen::from_str(input)
        .ok()
        .map(|kitchen| kitchen.fresh_ingredients())
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    Kitchen::from_str(input)
        .ok()
        .map(|kitchen| kitchen.total_fresh_ingredients())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_kitchen() -> Kitchen {
        Kitchen {
            fresh_ranges: vec![(3, 5), (10, 14), (12, 18), (16, 20)],
            ingredients: vec![1, 5, 8, 11, 17, 32],
        }
    }

    #[test]
    fn test_parse_kitchen() {
        assert_eq!(
            Kitchen::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_kitchen()),
        );
    }

    #[test]
    fn test_is_fresh() {
        let kitchen = example_kitchen();
        assert_eq!(kitchen.is_fresh(1), false);
        assert_eq!(kitchen.is_fresh(5), true);
        assert_eq!(kitchen.is_fresh(8), false);
        assert_eq!(kitchen.is_fresh(11), true);
        assert_eq!(kitchen.is_fresh(17), true);
        assert_eq!(kitchen.is_fresh(32), false);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
