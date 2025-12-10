use std::iter::repeat_with;
use std::str::FromStr;

advent_of_code::solution!(4);

const GRID_SIZE: usize = if cfg!(test) { 12 } else { 141 };
const TOP_LEFT: usize = GRID_SIZE + 1;
const BOTTOM_RIGHT: usize = (GRID_SIZE - 1) * GRID_SIZE;

#[derive(Debug, PartialEq)]
struct Grid {
    grid: Vec<bool>,
}

impl Grid {
    const fn neighbours(pos: usize) -> [usize; 8] {
        [
            pos - GRID_SIZE,
            pos - GRID_SIZE + 1,
            pos + 1,
            pos + GRID_SIZE + 1,
            pos + GRID_SIZE,
            pos + GRID_SIZE - 1,
            pos - 1,
            pos - GRID_SIZE - 1,
        ]
    }

    fn count_accessible_rolls(&self) -> usize {
        (TOP_LEFT..=BOTTOM_RIGHT)
            .filter(|pos| self.grid[*pos] && self.is_accessible_by_forklift(*pos))
            .count()
    }

    fn count_all_removable_rolls(mut self) -> usize {
        repeat_with(|| {
            (TOP_LEFT..=BOTTOM_RIGHT)
                .map(|pos| {
                    if self.grid[pos] && self.is_accessible_by_forklift(pos) {
                        self.grid[pos] = false;
                        return 1;
                    }
                    0
                })
                .sum()
        })
        .take_while(|removed: &usize| *removed > 0)
        .sum()
    }

    fn is_accessible_by_forklift(&self, pos: usize) -> bool {
        Self::neighbours(pos)
            .iter()
            .filter(|neighbour| self.grid[**neighbour])
            .count()
            < 4
    }
}

#[derive(Debug, PartialEq)]
struct ParseGridError;

impl FromStr for Grid {
    type Err = ParseGridError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = vec![false; GRID_SIZE * GRID_SIZE];
        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let pos = ((row + 1) * GRID_SIZE) + col + 1;
                grid[pos] = match ch {
                    '@' => true,
                    '.' => false,
                    _ => return Err(ParseGridError),
                };
            }
        }
        Ok(Self { grid })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Grid::from_str(input)
        .ok()
        .as_ref()
        .map(Grid::count_accessible_rolls)
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    Grid::from_str(input)
        .ok()
        .map(Grid::count_all_removable_rolls)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_grid() -> Grid {
        Grid {
            grid: vec![
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, true, true, false, true, true, true, true, false, false,
                false, true, true, true, false, true, false, true, false, true, true, false, false,
                true, true, true, true, true, false, true, false, true, true, false, false, true,
                false, true, true, true, true, false, false, true, false, false, false, true, true,
                false, true, true, true, true, false, true, true, false, false, false, true, true,
                true, true, true, true, true, false, true, false, false, false, true, false, true,
                false, true, false, true, true, true, false, false, true, false, true, true, true,
                false, true, true, true, true, false, false, false, true, true, true, true, true,
                true, true, true, false, false, false, true, false, true, false, true, true, true,
                false, true, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
        }
    }

    fn position(row: usize, col: usize) -> usize {
        ((row + 1) * GRID_SIZE) + col + 1
    }

    #[test]
    fn test_parse_grid() {
        assert_eq!(
            Grid::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_grid()),
        );
    }

    #[test]
    fn test_is_accessible_by_forklift() {
        let grid = example_grid();
        assert_eq!(grid.is_accessible_by_forklift(position(0, 2)), true);
        assert_eq!(grid.is_accessible_by_forklift(position(0, 3)), true);
        assert_eq!(grid.is_accessible_by_forklift(position(2, 3)), false);
        assert_eq!(grid.is_accessible_by_forklift(position(3, 1)), false);
        assert_eq!(grid.is_accessible_by_forklift(position(4, 9)), true);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
