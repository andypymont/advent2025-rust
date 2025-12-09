use std::str::FromStr;

advent_of_code::solution!(9);

#[derive(Clone, Copy, Debug, PartialEq)]
struct Tile {
    x: u64,
    y: u64,
}

#[derive(Debug, PartialEq)]
struct Rectangle {
    top_left: Tile,
    bottom_right: Tile,
}

impl Rectangle {
    const fn area(&self) -> u64 {
        (1 + self.top_left.x.abs_diff(self.bottom_right.x))
            * (1 + self.top_left.y.abs_diff(self.bottom_right.y))
    }

    fn from_tiles(first: Tile, second: Tile) -> Self {
        Self {
            top_left: Tile {
                x: first.x.min(second.x),
                y: first.y.min(second.y),
            },
            bottom_right: Tile {
                x: first.x.max(second.x),
                y: first.y.max(second.y),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
struct Polygon {
    tiles: Vec<Tile>,
}

impl Polygon {
    fn all_rectangles(&self) -> impl Iterator<Item = Rectangle> {
        self.tiles.iter().enumerate().flat_map(|(ix, a)| {
            self.tiles
                .iter()
                .skip(ix + 1)
                .map(|b| Rectangle::from_tiles(*a, *b))
        })
    }

    fn max_area(&self) -> Option<u64> {
        self.all_rectangles().map(|rect| rect.area()).max()
    }
}

#[derive(Debug, PartialEq)]
struct ParseTilesError;

impl FromStr for Polygon {
    type Err = ParseTilesError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut tiles = Vec::new();
        for line in input.lines() {
            let (x, y) = line.split_once(',').ok_or(ParseTilesError)?;
            let x = x.parse().map_err(|_| ParseTilesError)?;
            let y = y.parse().map_err(|_| ParseTilesError)?;
            tiles.push(Tile { x, y });
        }
        Ok(Self { tiles })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    Polygon::from_str(input)
        .ok()
        .as_ref()
        .map(Polygon::max_area)?
}

#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_polygon() -> Polygon {
        Polygon {
            tiles: vec![
                Tile { x: 7, y: 1 },
                Tile { x: 11, y: 1 },
                Tile { x: 11, y: 7 },
                Tile { x: 9, y: 7 },
                Tile { x: 9, y: 5 },
                Tile { x: 2, y: 5 },
                Tile { x: 2, y: 3 },
                Tile { x: 7, y: 3 },
            ],
        }
    }

    #[test]
    fn test_read_tiles() {
        assert_eq!(
            Polygon::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_polygon()),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
