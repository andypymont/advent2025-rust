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
    distances: Vec<u64>,
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

    fn connect_closest_boxes(&mut self, quantity: u64) {
        let mut distances: Vec<(usize, u64)> = self.distances.iter().copied().enumerate().collect();
        distances.sort_unstable_by(|(_, a), (_, b)| b.cmp(a));

        let mut completed = 0;
        while let Some((ix, _dist)) = distances.pop() {
            if !self.connections[ix] {
                self.connections[ix] = true;

                let a = ix / self.boxes.len();
                let b = ix % self.boxes.len();
                self.connections[(b * self.boxes.len()) + a] = true;

                completed += 1;

                if completed >= quantity {
                    return;
                }
            }
        }
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
        let mut distances = vec![0; boxes.len() * boxes.len()];
        for a in 0..boxes.len() {
            connections[(boxes.len() + 1) * a] = true;

            for b in (a + 1)..boxes.len() {
                let dist = straight_line_distance(boxes[a], boxes[b]);
                distances[(a * boxes.len()) + b] = dist;
                distances[(b * boxes.len()) + a] = dist;
            }
        }

        Ok(Self {
            boxes,
            connections,
            distances,
        })
    }
}

const CONNECTIONS_PART_ONE: u64 = if cfg!(test) { 10 } else { 1000 };

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Decorations::from_str(input).ok().map(|mut decorations| {
        decorations.connect_closest_boxes(CONNECTIONS_PART_ONE);
        let sizes = decorations.circuit_sizes();
        sizes[0] * sizes[1] * sizes[2]
    })
}

#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn part_two(_input: &str) -> Option<u64> {
    None
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
            distances: vec![
                0, 787, 908, 561, 723, 736, 1047, 321, 693, 391, 693, 1044, 709, 970, 413, 1103,
                927, 1290, 1191, 316, 787, 0, 1019, 1041, 471, 424, 783, 1024, 795, 628, 1046, 847,
                655, 1124, 600, 913, 1001, 979, 1103, 734, 908, 1019, 0, 507, 612, 670, 589, 790,
                347, 867, 834, 833, 812, 322, 818, 540, 671, 605, 352, 597, 561, 1041, 507, 0, 697,
                814, 837, 384, 524, 604, 503, 1076, 700, 492, 711, 941, 863, 1032, 811, 367, 723,
                471, 612, 697, 0, 373, 371, 844, 521, 491, 717, 844, 372, 661, 629, 708, 878, 639,
                681, 527, 736, 424, 670, 814, 373, 0, 648, 845, 411, 686, 1002, 495, 714, 867, 433,
                511, 603, 735, 796, 533, 1047, 783, 589, 837, 371, 648, 0, 1100, 690, 788, 817,
                1020, 535, 550, 968, 740, 1048, 379, 459, 809, 321, 1024, 790, 384, 844, 845, 1100,
                0, 630, 609, 712, 1057, 860, 863, 543, 1069, 841, 1295, 1121, 328, 693, 795, 347,
                524, 521, 411, 690, 630, 0, 738, 888, 566, 789, 610, 493, 444, 417, 740, 621, 387,
                391, 628, 867, 604, 491, 686, 788, 609, 738, 0, 476, 1130, 338, 842, 597, 1095,
                1082, 1106, 1056, 433, 693, 1046, 834, 503, 717, 1002, 817, 712, 888, 476, 0, 1410,
                458, 646, 960, 1256, 1284, 1145, 996, 650, 1044, 847, 833, 1076, 844, 495, 1020,
                1057, 566, 1130, 1410, 0, 1194, 1131, 652, 407, 344, 927, 966, 832, 709, 655, 812,
                700, 372, 714, 535, 860, 789, 338, 458, 1194, 0, 716, 816, 1059, 1181, 900, 889,
                626, 970, 1124, 322, 492, 661, 867, 550, 863, 610, 842, 646, 1131, 716, 0, 1000,
                830, 981, 683, 411, 705, 413, 600, 818, 711, 629, 433, 968, 543, 493, 597, 960,
                652, 816, 1000, 0, 802, 609, 1113, 1075, 350, 1103, 913, 540, 941, 708, 511, 740,
                1069, 444, 1095, 1256, 407, 1059, 830, 802, 0, 455, 566, 582, 814, 927, 1001, 671,
                863, 878, 603, 1048, 841, 417, 1082, 1284, 344, 1181, 981, 609, 455, 0, 983, 902,
                692, 1290, 979, 605, 1032, 639, 735, 379, 1295, 740, 1106, 1145, 927, 900, 683,
                1113, 566, 983, 0, 333, 1007, 1191, 1103, 352, 811, 681, 796, 459, 1121, 621, 1056,
                996, 966, 889, 411, 1075, 582, 902, 333, 0, 888, 316, 734, 597, 367, 527, 533, 809,
                328, 387, 433, 650, 832, 626, 705, 350, 814, 692, 1007, 888, 0,
            ],
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
