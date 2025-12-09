advent_of_code::solution!(9);

type Tile = (u64, u64);

#[derive(Debug, PartialEq)]
struct ParseTilesError;

fn read_tiles(input: &str) -> Result<Vec<Tile>, ParseTilesError> {
    let mut tiles = Vec::new();
    for line in input.lines() {
        let (x, y) = line.split_once(',').ok_or(ParseTilesError)?;
        let x = x.parse().map_err(|_| ParseTilesError)?;
        let y = y.parse().map_err(|_| ParseTilesError)?;
        tiles.push((x, y));
    }
    Ok(tiles)
}

const fn area(first: Tile, second: Tile) -> u64 {
    (first.0.abs_diff(second.0) + 1) * (first.1.abs_diff(second.1) + 1)
}

fn max_area(tiles: &[Tile]) -> Option<u64> {
    (0..tiles.len())
        .flat_map(|a| (0..tiles.len()).map(move |b| area(tiles[a], tiles[b])))
        .max()
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    read_tiles(input).ok().map(|tiles| max_area(&tiles))?
}

#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_tiles() -> Vec<Tile> {
        vec![
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ]
    }

    #[test]
    fn test_read_tiles() {
        assert_eq!(
            read_tiles(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_tiles()),
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
