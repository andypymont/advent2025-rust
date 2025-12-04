use std::str::FromStr;

advent_of_code::solution!(4);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

const COMPASS: [Direction; 8] = [
    Direction::North,
    Direction::Northeast,
    Direction::East,
    Direction::Southeast,
    Direction::South,
    Direction::Southwest,
    Direction::West,
    Direction::Northwest,
];

const GRID_SIZE: usize = if cfg!(test) { 10 } else { 139 };

#[derive(Debug, PartialEq)]
struct Grid {
    grid: Vec<bool>,
}

impl Grid {
    fn neighbours(pos: usize) -> impl Iterator<Item = usize> {
        COMPASS
            .iter()
            .filter_map(move |dir| Self::step_in_direction(pos, *dir))
    }

    fn step_in_direction(pos: usize, dir: Direction) -> Option<usize> {
        let row = pos / GRID_SIZE;
        let col = pos % GRID_SIZE;

        let row = match dir {
            Direction::North | Direction::Northwest | Direction::Northeast => row.checked_sub(1),
            Direction::South | Direction::Southwest | Direction::Southeast => {
                let r = row + 1;
                if r >= GRID_SIZE { None } else { Some(r) }
            }
            _ => Some(row),
        }?;
        let col = match dir {
            Direction::West | Direction::Northwest | Direction::Southwest => col.checked_sub(1),
            Direction::East | Direction::Northeast | Direction::Southeast => {
                let c = col + 1;
                if c >= GRID_SIZE { None } else { Some(c) }
            }
            _ => Some(col),
        }?;

        Some((row * GRID_SIZE) + col)
    }

    fn count_accessible_rolls(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .filter(|(pos, is_paper)| **is_paper && self.is_accessible_by_forklift(*pos))
            .count()
    }

    fn count_all_removable_rolls(mut self) -> usize {
        let mut removed = 0;
        let mut just_removed = 1;

        while just_removed > 0 {
            just_removed = 0;

            for pos in 0..(GRID_SIZE * GRID_SIZE) {
                if self.grid[pos] && self.is_accessible_by_forklift(pos) {
                    self.grid[pos] = false;
                    removed += 1;
                    just_removed += 1;
                }
            }
        }

        removed
    }

    fn is_accessible_by_forklift(&self, pos: usize) -> bool {
        Self::neighbours(pos)
            .filter(|other| self.grid[*other])
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
                grid[(row * GRID_SIZE) + col] = match ch {
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
        .map(|grid| grid.count_accessible_rolls())
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
                false, false, true, true, false, true, true, true, true, false, true, true, true,
                false, true, false, true, false, true, true, true, true, true, true, true, false,
                true, false, true, true, true, false, true, true, true, true, false, false, true,
                false, true, true, false, true, true, true, true, false, true, true, false, true,
                true, true, true, true, true, true, false, true, false, true, false, true, false,
                true, false, true, true, true, true, false, true, true, true, false, true, true,
                true, true, false, true, true, true, true, true, true, true, true, false, true,
                false, true, false, true, true, true, false, true, false,
            ],
        }
    }

    fn position(row: usize, col: usize) -> usize {
        (row * GRID_SIZE) + col
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
