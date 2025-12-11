use std::collections::BTreeMap;
use std::str::FromStr;

advent_of_code::solution!(11);

#[derive(Debug, PartialEq)]
struct Network {
    start: usize,
    output: usize,
    connections: Vec<Vec<usize>>,
}

impl Network {
    fn count_all_paths(&self) -> u64 {
        let mut paths = 0;
        let mut queue = vec![self.start];

        while let Some(node) = queue.pop() {
            if node == self.output {
                paths += 1;
                continue;
            }

            queue.extend(self.connections[node].iter());
        }

        paths
    }
}

#[derive(Debug, PartialEq)]
struct ParseNetworkError;

impl FromStr for Network {
    type Err = ParseNetworkError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut nodes = BTreeMap::new();
        let mut connections = Vec::new();

        let mut start = Err(ParseNetworkError);
        let mut output = Err(ParseNetworkError);

        for line in input.lines() {
            let (node, connected) = line.split_once(": ").ok_or(ParseNetworkError)?;

            let node_ix = *nodes.entry(node).or_insert_with(|| {
                connections.push(Vec::new());
                connections.len() - 1
            });

            if node == "you" {
                start = Ok(node_ix);
            }

            for other in connected.split_whitespace() {
                let other_ix = *nodes.entry(other).or_insert_with(|| {
                    connections.push(Vec::new());
                    connections.len() - 1
                });
                if other == "out" {
                    output = Ok(other_ix);
                }

                connections[node_ix].push(other_ix);
            }
        }

        let output = output?;
        start.map(|start| Self {
            start,
            output,
            connections,
        })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    Network::from_str(input)
        .ok()
        .as_ref()
        .map(Network::count_all_paths)
}

#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_network() -> Network {
        Network {
            start: 1,
            output: 9,
            connections: vec![
                vec![1, 2],
                vec![3, 4],
                vec![4, 7, 10],
                vec![5, 6],
                vec![5, 6, 7],
                vec![8],
                vec![9],
                vec![9],
                vec![9],
                vec![],
                vec![9],
            ],
        }
    }

    #[test]
    fn test_parse_network() {
        assert_eq!(
            Network::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_network()),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
