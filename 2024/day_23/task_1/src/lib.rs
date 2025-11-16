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

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Party {
    computers: [Computer; 3],
}

impl Party {
    fn new(computer1: Computer, computer2: Computer, computer3: Computer) -> Self {
        let mut attendees = [computer1, computer2, computer3];
        attendees.sort();
        Self {
            computers: attendees,
        }
    }
}

pub fn solve(input: &str) -> String {
    let network_map = build_network_map(input);
    let all_sets = find_sets(&network_map);
    let possible_sets = find_chief_historian(&all_sets);

    let result = possible_sets.len();

    return format!(
        "The chief historian could be attending one of {} LAN parties",
        result
    );
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

fn find_sets(network_map: &HashMap<Computer, HashSet<Computer>>) -> HashSet<Party> {
    let mut sets = HashSet::new();

    for (computer1, connections1) in network_map {
        for computer2 in connections1 {
            let connections2 = network_map
                .get(computer2)
                .unwrap_or(&HashSet::new())
                .to_owned();
            for computer3 in connections2 {
                let connections3 = network_map
                    .get(&computer3)
                    .unwrap_or(&HashSet::new())
                    .to_owned();
                if connections3.contains(computer1) {
                    sets.insert(Party::new(computer1.clone(), computer2.clone(), computer3));
                }
            }
        }
    }

    return sets;
}

fn find_chief_historian(sets: &HashSet<Party>) -> HashSet<Party> {
    sets.iter()
        .filter(|p| p.computers.iter().any(|c| c.name[0] == 't'))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let network_map = build_network_map(&input);
        let all_sets = find_sets(&network_map);
        let possible_sets = find_chief_historian(&all_sets);
        assert_eq!(possible_sets.len(), 7);
    }
}
