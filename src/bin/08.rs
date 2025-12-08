use std::iter::repeat_with;
use std::str::FromStr;

advent_of_code::solution!(8);

type JunctionBox = (u64, u64, u64);

const fn straight_line_distance(first: JunctionBox, second: JunctionBox) -> u64 {
    let x = first.0.abs_diff(second.0);
    let y = first.1.abs_diff(second.1);
    let z = first.2.abs_diff(second.2);
    ((x * x) + (y * y) + (z * z)).isqrt()
}

#[derive(Debug, PartialEq)]
struct Decorations {
    boxes: Vec<JunctionBox>,
    connections: Vec<bool>,
    nearest_pairs: Vec<usize>,
}

impl Decorations {
    fn circuit_sizes(&self) -> Vec<usize> {
        let mut sizes = Vec::new();
        let mut visited = vec![false; self.boxes.len()];

        for ix in 0..self.boxes.len() {
            if visited[ix] {
                continue;
            }

            let mut size = 0;
            let mut queue = Vec::new();
            queue.push(ix);

            while let Some(ix) = queue.pop() {
                if visited[ix] {
                    continue;
                }

                visited[ix] = true;
                size += 1;

                for other in 0..self.boxes.len() {
                    if self.connections[(ix * self.boxes.len()) + other] {
                        queue.push(other);
                    }
                }
            }

            sizes.push(size);
        }

        sizes.sort_unstable_by(|a, b| b.cmp(a));
        sizes
    }

    fn connect_closest_pair(&mut self) -> Option<(JunctionBox, JunctionBox)> {
        while let Some(ix) = self.nearest_pairs.pop() {
            if self.connections[ix] {
                continue;
            }

            self.connections[ix] = true;
            let a = ix / self.boxes.len();
            let b = ix % self.boxes.len();
            self.connections[(b * self.boxes.len()) + a] = true;

            return Some((self.boxes[a], self.boxes[b]));
        }

        None
    }

    fn connect_closest_boxes(&mut self, quantity: usize) -> usize {
        repeat_with(|| self.connect_closest_pair())
            .flatten()
            .take(quantity)
            .count()
    }

    fn final_connection(&mut self) -> Option<(JunctionBox, JunctionBox)> {
        while let Some(pair) = self.connect_closest_pair() {
            if self.circuit_sizes().len() > 1 {
                continue;
            }
            return Some(pair);
        }

        None
    }
}

#[derive(Debug, PartialEq)]
struct ParseDecorationsError;

impl FromStr for Decorations {
    type Err = ParseDecorationsError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut boxes = Vec::new();

        for line in input.lines() {
            let mut coords = line
                .split(',')
                .map(|coord| coord.parse().map_err(|_| ParseDecorationsError));

            let x = coords.next().ok_or(ParseDecorationsError)??;
            let y = coords.next().ok_or(ParseDecorationsError)??;
            let z = coords.next().ok_or(ParseDecorationsError)??;
            if coords.next().is_some() {
                return Err(ParseDecorationsError)?;
            }

            boxes.push((x, y, z));
        }

        let mut connections = vec![false; boxes.len() * boxes.len()];
        let mut nearest_pairs = Vec::new();
        let mut distances = vec![0; boxes.len() * boxes.len()];

        for a in 0..boxes.len() {
            connections[(boxes.len() + 1) * a] = true;
            for b in (a + 1)..boxes.len() {
                let pos = (a * boxes.len()) + b;
                nearest_pairs.push(pos);
                distances[pos] = straight_line_distance(boxes[a], boxes[b]);
            }
        }

        nearest_pairs.sort_unstable_by(|x, y| distances[*y].cmp(&distances[*x]));
        Ok(Self {
            boxes,
            connections,
            nearest_pairs,
        })
    }
}

const CONNECTIONS_PART_ONE: usize = if cfg!(test) { 10 } else { 1000 };

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Decorations::from_str(input).ok().map(|mut decorations| {
        decorations.connect_closest_boxes(CONNECTIONS_PART_ONE);
        let sizes = decorations.circuit_sizes();
        sizes[0] * sizes[1] * sizes[2]
    })
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    let mut decorations = Decorations::from_str(input).ok()?;
    decorations.final_connection().map(|(a, b)| a.0 * b.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_decorations() -> Decorations {
        let mut connections = vec![false; 20 * 20];
        for ix in 0..20 {
            connections[ix * 21] = true;
        }

        Decorations {
            boxes: vec![
                (162, 817, 812),
                (57, 618, 57),
                (906, 360, 560),
                (592, 479, 940),
                (352, 342, 300),
                (466, 668, 158),
                (542, 29, 236),
                (431, 825, 988),
                (739, 650, 466),
                (52, 470, 668),
                (216, 146, 977),
                (819, 987, 18),
                (117, 168, 530),
                (805, 96, 715),
                (346, 949, 466),
                (970, 615, 88),
                (941, 993, 340),
                (862, 61, 35),
                (984, 92, 344),
                (425, 690, 689),
            ],
            connections,
            nearest_pairs: vec![
                211, 157, 17, 216, 215, 232, 18, 256, 217, 233, 191, 33, 158, 297, 197, 15, 38,
                127, 195, 196, 71, 298, 155, 255, 151, 198, 136, 6, 30, 11, 23, 77, 27, 131, 22,
                359, 110, 36, 274, 218, 337, 276, 37, 13, 134, 238, 214, 75, 237, 16, 35, 2, 338,
                257, 258, 170, 379, 96, 49, 113, 153, 76, 152, 31, 107, 91, 87, 193, 156, 66, 50,
                51, 239, 275, 54, 130, 254, 319, 65, 52, 78, 139, 295, 118, 28, 47, 172, 129, 1,
                26, 135, 177, 169, 5, 117, 39, 4, 90, 253, 112, 150, 74, 12, 95, 279, 72, 64, 10,
                8, 339, 128, 109, 277, 98, 56, 45, 93, 32, 234, 219, 106, 213, 97, 148, 94, 29,
                259, 178, 44, 173, 296, 149, 57, 69, 116, 34, 194, 59, 46, 318, 317, 171, 3, 133,
                154, 55, 132, 119, 99, 68, 88, 115, 43, 70, 111, 174, 73, 89, 190, 24, 138, 212,
                316, 175, 199, 114, 25, 176, 14, 278, 108, 235, 9, 179, 67, 137, 85, 92, 86, 79,
                58, 299, 48, 236, 192, 358, 159, 53, 7, 19,
            ],
        }
    }

    #[test]
    fn test_parse_decorations() {
        let example = example_decorations();
        let parsed = Decorations::from_str(&advent_of_code::template::read_file("examples", DAY))
            .expect("Error during parsing");
        assert_eq!(parsed.boxes, example.boxes);
        assert_eq!(parsed.connections, example.connections);
    }

    #[test]
    fn test_connect_closest_boxes() {
        let mut decorations = example_decorations();
        decorations.connect_closest_boxes(4);
        assert_eq!(decorations.connections[(0 * 20) + 7], true);
        assert_eq!(decorations.connections[(0 * 20) + 19], true);
        assert_eq!(decorations.connections[(2 * 20) + 13], true);
        assert_eq!(decorations.connections[(8 * 20) + 16], false);
        assert_eq!(decorations.connections[(14 * 20) + 15], false);
    }

    #[test]
    fn test_circuit_sizes() {
        let mut decorations = example_decorations();
        assert_eq!(decorations.circuit_sizes(), vec![1; 20]);

        decorations.connect_closest_boxes(4);
        assert_eq!(
            decorations.circuit_sizes(),
            vec![3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        );

        decorations.connect_closest_boxes(6);
        assert_eq!(
            decorations.circuit_sizes(),
            vec![5, 4, 2, 2, 1, 1, 1, 1, 1, 1, 1],
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_final_connection() {
        assert_eq!(
            example_decorations().final_connection(),
            Some(((216, 146, 977), (117, 168, 530))),
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25_272));
    }
}
