use std::str::FromStr;

advent_of_code::solution!(9);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct Tile {
    x: u64,
    y: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct LineSegment {
    first: Tile,
    second: Tile,
}

impl LineSegment {
    const fn is_vertical(&self) -> bool {
        self.first.x == self.second.x
    }

    fn from_tiles(first: Tile, second: Tile) -> Self {
        if first > second {
            Self {
                first: second,
                second: first,
            }
        } else {
            Self { first, second }
        }
    }
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

    fn center_inside_polygon(&self, polygon: &Polygon) -> bool {
        // raycasting method: check if a horizontal line through the center passes through an odd
        // numnber of polygon line segments; use doubled coordinates for the calculation to avoid
        // floating point arithmetic
        let center_x = self.top_left.x + self.bottom_right.x;
        let center_y = self.top_left.y + self.bottom_right.y;

        polygon
            .line_segments()
            .filter(LineSegment::is_vertical)
            .filter(|segment| {
                segment.first.x * 2 > center_x
                    && center_y > (segment.first.y * 2)
                    && center_y < (segment.second.y * 2)
            })
            .count()
            % 2
            == 1
    }

    fn is_contained_by(&self, polygon: &Polygon) -> bool {
        polygon
            .line_segments()
            .all(|segment| !self.is_crossed_by_segment(segment))
            && self.center_inside_polygon(polygon)
    }

    fn is_crossed_by_segment(&self, segment: LineSegment) -> bool {
        if segment.is_vertical() {
            segment.first.x > self.top_left.x
                && segment.first.x < self.bottom_right.x
                && self.top_left.y.max(segment.first.y) < self.bottom_right.y.min(segment.second.y)
        } else {
            segment.first.y > self.top_left.y
                && segment.first.y < self.bottom_right.y
                && self.top_left.x.max(segment.first.x) < self.bottom_right.x.min(segment.second.x)
        }
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

    fn contained_rectangles(&self) -> impl Iterator<Item = Rectangle> {
        self.all_rectangles()
            .filter(move |rect| rect.is_contained_by(self))
    }

    fn line_segments(&self) -> impl Iterator<Item = LineSegment> {
        (0..self.tiles.len()).map(|ix| {
            LineSegment::from_tiles(self.tiles[ix], self.tiles[(ix + 1) % self.tiles.len()])
        })
    }

    fn max_area(&self) -> Option<u64> {
        self.all_rectangles().map(|rect| rect.area()).max()
    }

    fn max_contained_area(&self) -> Option<u64> {
        self.contained_rectangles().map(|rect| rect.area()).max()
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

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    Polygon::from_str(input)
        .ok()
        .as_ref()
        .map(Polygon::max_contained_area)?
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
    fn test_contained_rectangles() {
        assert_eq!(
            example_polygon()
                .contained_rectangles()
                .collect::<Vec<Rectangle>>(),
            vec![
                Rectangle {
                    top_left: Tile { x: 7, y: 1 },
                    bottom_right: Tile { x: 9, y: 5 }
                },
                Rectangle {
                    top_left: Tile { x: 7, y: 1 },
                    bottom_right: Tile { x: 7, y: 3 }
                },
                Rectangle {
                    top_left: Tile { x: 9, y: 1 },
                    bottom_right: Tile { x: 11, y: 7 }
                },
                Rectangle {
                    top_left: Tile { x: 9, y: 1 },
                    bottom_right: Tile { x: 11, y: 5 }
                },
                Rectangle {
                    top_left: Tile { x: 7, y: 1 },
                    bottom_right: Tile { x: 11, y: 3 }
                },
                Rectangle {
                    top_left: Tile { x: 9, y: 5 },
                    bottom_right: Tile { x: 11, y: 7 }
                },
                Rectangle {
                    top_left: Tile { x: 9, y: 5 },
                    bottom_right: Tile { x: 9, y: 7 }
                },
                Rectangle {
                    top_left: Tile { x: 2, y: 5 },
                    bottom_right: Tile { x: 9, y: 5 }
                },
                Rectangle {
                    top_left: Tile { x: 2, y: 3 },
                    bottom_right: Tile { x: 9, y: 5 }
                },
                Rectangle {
                    top_left: Tile { x: 7, y: 3 },
                    bottom_right: Tile { x: 9, y: 5 }
                },
                Rectangle {
                    top_left: Tile { x: 2, y: 3 },
                    bottom_right: Tile { x: 2, y: 5 }
                },
                Rectangle {
                    top_left: Tile { x: 2, y: 3 },
                    bottom_right: Tile { x: 7, y: 5 }
                },
                Rectangle {
                    top_left: Tile { x: 2, y: 3 },
                    bottom_right: Tile { x: 7, y: 3 }
                }
            ]
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
