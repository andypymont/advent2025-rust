use std::collections::BTreeMap;
use std::str::FromStr;

advent_of_code::solution!(11);

#[derive(Debug, PartialEq)]
struct Network {
    nodes: Vec<usize>,
    connections: Vec<Vec<usize>>,
}

impl Network {
    fn count_all_paths(&self, start: &str) -> usize {
        let finish = Self::node_key("out");
        let mut cache = BTreeMap::new();
        cache.insert(finish, 1);
        self.count_paths(&mut cache, Self::node_key(start))
    }

    fn count_all_paths_visiting(&self, start: &str, check_a: &str, check_b: &str) -> usize {
        let finish = Self::node_key("out");
        let mut cache = BTreeMap::new();
        cache.insert((finish, false, false), 0);
        cache.insert((finish, true, false), 0);
        cache.insert((finish, false, true), 0);
        cache.insert((finish, true, true), 1);

        self.count_paths_visiting(
            &mut cache,
            Self::node_key(start),
            Self::node_key(check_a),
            Self::node_key(check_b),
            false,
            false,
        )
    }

    fn count_paths(&self, cache: &mut BTreeMap<usize, usize>, node: usize) -> usize {
        if let Some(cached) = cache.get(&node) {
            return *cached;
        }

        let result = self.connections[self.nodes[node]]
            .iter()
            .map(|other| self.count_paths(cache, *other))
            .sum();
        cache.insert(node, result);
        result
    }

    fn count_paths_visiting(
        &self,
        cache: &mut BTreeMap<(usize, bool, bool), usize>,
        node: usize,
        check_a: usize,
        check_b: usize,
        visited_a: bool,
        visited_b: bool,
    ) -> usize {
        let visited_a = visited_a || (node == check_a);
        let visited_b = visited_b || (node == check_b);

        if let Some(cached) = cache.get(&(node, visited_a, visited_b)) {
            return *cached;
        }

        let result = self.connections[self.nodes[node]]
            .iter()
            .map(|other| {
                self.count_paths_visiting(cache, *other, check_a, check_b, visited_a, visited_b)
            })
            .sum();
        cache.insert((node, visited_a, visited_b), result);
        result
    }

    fn node_key(node: &str) -> usize {
        node.chars()
            .filter_map(|ch| {
                ch.to_digit(36)
                    .and_then(|d| usize::try_from(d).ok())
                    .and_then(|d| d.checked_sub(10))
            })
            .take(3)
            .fold(0, |key, digit| (key * 26) + digit)
    }
}

#[derive(Debug, PartialEq)]
struct ParseNetworkError;

impl FromStr for Network {
    type Err = ParseNetworkError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut nodes = vec![0; 26 * 26 * 26];
        let mut connections = Vec::new();
        connections.push(Vec::new());

        for line in input.lines() {
            let (node, connected) = line.split_once(": ").ok_or(ParseNetworkError)?;

            let node_key = Self::node_key(node);
            let node_ix = {
                let node_ix = nodes[node_key];
                if node_ix == 0 {
                    connections.push(Vec::new());
                    nodes[node_key] = connections.len() - 1;
                    connections.len() - 1
                } else {
                    node_ix
                }
            };

            connections[node_ix].extend(connected.split_whitespace().map(Self::node_key));
        }

        Ok(Self { nodes, connections })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Network::from_str(input)
        .ok()
        .map(|network| network.count_all_paths("you"))
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    Network::from_str(input)
        .ok()
        .map(|network| network.count_all_paths_visiting("svr", "dac", "fft"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_key() {
        assert_eq!(Network::node_key("aaa"), 0);
        assert_eq!(Network::node_key("eee"), 2812);
        assert_eq!(Network::node_key("iii"), 5624);
        assert_eq!(Network::node_key("out"), 10003);
    }

    fn example_network() -> Network {
        let mut nodes = vec![0; 26 * 26 * 26];
        nodes[Network::node_key("you")] = 1;
        nodes[Network::node_key("svr")] = 2;
        nodes[Network::node_key("aaa")] = 3;
        nodes[Network::node_key("fft")] = 4;
        nodes[Network::node_key("bbb")] = 5;
        nodes[Network::node_key("tty")] = 6;
        nodes[Network::node_key("ccc")] = 7;
        nodes[Network::node_key("ddd")] = 8;
        nodes[Network::node_key("hub")] = 9;
        nodes[Network::node_key("eee")] = 10;
        nodes[Network::node_key("dac")] = 11;
        nodes[Network::node_key("fff")] = 12;
        nodes[Network::node_key("ggg")] = 13;
        nodes[Network::node_key("hhh")] = 14;

        Network {
            nodes,
            connections: vec![
                vec![],
                vec![Network::node_key("aaa"), Network::node_key("bbb")],
                vec![Network::node_key("aaa"), Network::node_key("bbb")],
                vec![Network::node_key("fft")],
                vec![Network::node_key("ccc")],
                vec![Network::node_key("tty")],
                vec![Network::node_key("ccc")],
                vec![Network::node_key("ddd"), Network::node_key("eee")],
                vec![Network::node_key("hub")],
                vec![Network::node_key("fff")],
                vec![Network::node_key("dac")],
                vec![Network::node_key("fff")],
                vec![Network::node_key("ggg"), Network::node_key("hhh")],
                vec![Network::node_key("out")],
                vec![Network::node_key("out")],
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
