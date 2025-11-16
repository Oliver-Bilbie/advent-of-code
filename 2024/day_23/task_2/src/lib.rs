use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, PartialOrd, Ord, Clone, Debug)]
struct Computer {
    name: [char; 2],
}

impl Computer {
    fn new(name: &str) -> Self {
        Self {
            name: [name.chars().nth(0).unwrap(), name.chars().nth(1).unwrap()],
        }
    }
}

impl std::fmt::Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.name[0], self.name[1])
    }
}

pub fn solve(input: &str) -> String {
    let network_map = build_network_map(input);
    let party = find_lan_party(&network_map).expect("no lan parties were found");
    let password = party
        .iter()
        .map(|c| format!("{}{}", c.name[0], c.name[1]))
        .collect::<Vec<String>>()
        .join(",");

    return format!("The password is: {}", password);
}

fn build_network_map(input: &str) -> HashMap<Computer, HashSet<Computer>> {
    let mut network_map = HashMap::new();

    for line in input.lines() {
        let computer1 = Computer::new(&line[0..2]);
        let computer2 = Computer::new(&line[3..5]);

        network_map
            .entry(computer1.clone())
            .or_insert(HashSet::new())
            .insert(computer2.clone());

        network_map
            .entry(computer2.clone())
            .or_insert(HashSet::new())
            .insert(computer1.clone());
    }

    return network_map;
}

fn find_lan_party(network_map: &HashMap<Computer, HashSet<Computer>>) -> Option<Vec<Computer>> {
    // From examining the input, we know that all computers are connected to the same number of
    // devices
    let network_size = network_map.values().nth(0).unwrap().len() - 1;

    for (computer, connected) in network_map {
        if connected
            .iter()
            .filter(|&next_computer| {
                let next_connections = network_map.get(next_computer).unwrap();
                let overlap = next_connections
                    .iter()
                    .filter(|c| connected.contains(c))
                    .count();

                overlap == network_size - 1
            })
            .count()
            == network_size
        {
            // Remove the connected computer which does not overlap with all of the others
            let mut result = connected
                .iter()
                .filter(|next_computer| {
                    let next_connections = network_map.get(next_computer).unwrap();
                    connected
                        .iter()
                        .filter(|c| c == next_computer || next_connections.contains(c))
                        .count()
                        == network_size
                })
                .cloned()
                .collect::<Vec<Computer>>();
            // The source computer is not connected to itself, so we must add it to the list
            result.push(computer.to_owned());
            result.sort();
            return Some(result);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let network_map = build_network_map(&input);
        let party = find_lan_party(&network_map).expect("no lan parties were found");
        let password = party
            .iter()
            .map(|c| format!("{}{}", c.name[0], c.name[1]))
            .collect::<Vec<String>>()
            .join(",");
        assert_eq!(password, "co,de,ka,ta");
    }
}
