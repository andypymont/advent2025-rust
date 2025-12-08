use std::collections::VecDeque;
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
    nearest_pairs: VecDeque<usize>,
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
            let mut queue = VecDeque::new();
            queue.push_back(ix);

            while let Some(ix) = queue.pop_front() {
                if visited[ix] {
                    continue;
                }

                visited[ix] = true;
                size += 1;

                for other in 0..self.boxes.len() {
                    if self.connections[(ix * self.boxes.len()) + other] {
                        queue.push_back(other);
                    }
                }
            }

            sizes.push(size);
        }

        sizes.sort_unstable_by(|a, b| b.cmp(a));
        sizes
    }

    fn connect_closest_pair(&mut self) -> Option<(JunctionBox, JunctionBox)> {
        while let Some(ix) = self.nearest_pairs.pop_front() {
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

        for a in 0..boxes.len() {
            connections[(boxes.len() + 1) * a] = true;
            for b in (a + 1)..boxes.len() {
                nearest_pairs.push((a * boxes.len()) + b);
            }
        }
        nearest_pairs.sort_unstable_by_key(|ix| {
            let a = ix / boxes.len();
            let b = ix % boxes.len();
            straight_line_distance(boxes[a], boxes[b])
        });
        let nearest_pairs = nearest_pairs.into();

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
                19, 7, 53, 159, 358, 192, 236, 48, 299, 58, 79, 86, 92, 85, 137, 67, 179, 9, 235,
                108, 278, 14, 176, 25, 114, 199, 175, 316, 212, 138, 24, 190, 89, 73, 174, 111, 70,
                43, 115, 88, 68, 99, 119, 132, 55, 154, 133, 3, 171, 317, 318, 46, 59, 194, 34,
                116, 69, 57, 149, 296, 173, 44, 178, 259, 29, 94, 148, 97, 213, 106, 219, 234, 32,
                93, 45, 56, 98, 277, 109, 128, 339, 8, 10, 64, 72, 279, 95, 12, 74, 150, 112, 253,
                90, 4, 39, 117, 5, 169, 177, 135, 26, 1, 129, 172, 47, 28, 118, 295, 139, 78, 52,
                65, 319, 254, 130, 54, 275, 239, 51, 50, 66, 156, 193, 87, 91, 107, 31, 152, 76,
                153, 113, 49, 96, 379, 170, 258, 257, 338, 2, 35, 16, 237, 75, 214, 238, 134, 13,
                37, 276, 337, 218, 274, 36, 110, 359, 22, 131, 27, 77, 23, 11, 30, 6, 136, 198,
                151, 255, 155, 298, 71, 196, 195, 127, 38, 15, 197, 297, 158, 33, 191, 233, 217,
                256, 18, 232, 215, 216, 17, 157, 211,
            ]
            .into(),
        }
    }

    #[test]
    fn test_parse_decorations() {
        assert_eq!(
            Decorations::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_decorations()),
        );
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
