use std::collections::{HashMap, VecDeque};
use wasm_bindgen::prelude::*;

struct Pulse {
    is_high: bool,
    location: String,
    prev_location: String,
}
impl Pulse {
    fn process_pulse(self, modules: &mut HashMap<String, Module>) -> Vec<Pulse> {
        if let Some(module) = modules.get_mut(&self.location) {
            let mut next_pulses = Vec::<Pulse>::new();
            match module {
                Module::FlipFlop(m) => {
                    next_pulses.extend(m.process_pulse(self));
                }
                Module::Conjunction(m) => {
                    next_pulses.extend(m.process_pulse(self));
                }
                Module::Broadcaster(m) => {
                    next_pulses.extend(m.process_pulse(self));
                }
            };
            return next_pulses;
        }
        return vec![];
    }
}

enum Module {
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
    Broadcaster(BroadcasterModule),
}
impl Module {
    fn from_str(input: &str) -> (String, Self) {
        let (name, destinations_str) = input.split_once(" -> ").unwrap();
        if name == "broadcaster" {
            (
                name.to_string(),
                Module::Broadcaster(BroadcasterModule::from_str(destinations_str)),
            )
        } else if name.chars().nth(0).unwrap() == '%' {
            (
                name[1..].to_string(),
                Module::FlipFlop(FlipFlopModule::from_str(destinations_str)),
            )
        } else if name.chars().nth(0).unwrap() == '&' {
            (
                name[1..].to_string(),
                Module::Conjunction(ConjunctionModule::from_str(destinations_str)),
            )
        } else {
            panic!("{} is not a valid module name", name);
        }
    }
}

struct FlipFlopModule {
    is_on: bool,
    destinations: Vec<String>,
}
impl FlipFlopModule {
    fn from_str(input: &str) -> Self {
        Self {
            is_on: false,
            destinations: input.split(", ").map(|s| s.to_string()).collect(),
        }
    }

    fn process_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        if pulse.is_high {
            return vec![];
        }

        self.is_on = !self.is_on;
        self.destinations
            .iter()
            .map(|d| Pulse {
                is_high: self.is_on,
                location: d.clone(),
                prev_location: pulse.location.clone(),
            })
            .collect()
    }
}

struct ConjunctionModule {
    last_seen: HashMap<String, bool>,
    destinations: Vec<String>,
}
impl ConjunctionModule {
    fn from_str(input: &str) -> Self {
        Self {
            last_seen: HashMap::<String, bool>::new(),
            destinations: input.split(", ").map(|s| s.to_string()).collect(),
        }
    }

    fn process_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        let last_from_src = self.last_seen.get_mut(&pulse.prev_location).unwrap();
        *last_from_src = pulse.is_high;
        let all_high = self.last_seen.values().all(|is_high| *is_high);

        self.destinations
            .iter()
            .map(|d| Pulse {
                is_high: !all_high,
                location: d.clone(),
                prev_location: pulse.location.clone(),
            })
            .collect()
    }
}

struct BroadcasterModule {
    destinations: Vec<String>,
}
impl BroadcasterModule {
    fn from_str(input: &str) -> Self {
        Self {
            destinations: input.split(", ").map(|s| s.to_string()).collect(),
        }
    }

    fn process_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        self.destinations
            .iter()
            .map(|d| Pulse {
                is_high: pulse.is_high.clone(),
                location: d.clone(),
                prev_location: pulse.location.clone(),
            })
            .collect()
    }
}

fn read_modules(input: &str) -> HashMap<String, Module> {
    let mut modules = HashMap::<String, Module>::new();
    for line in input.lines() {
        let (name, module) = Module::from_str(&line);
        modules.insert(name, module);
    }

    // find the input modules for all conjunction modules
    let mut conjunction_inputs = HashMap::<String, Vec<String>>::new();
    for (name, module) in modules.iter() {
        let destinations = match module {
            Module::FlipFlop(m) => &m.destinations,
            Module::Conjunction(m) => &m.destinations,
            Module::Broadcaster(m) => &m.destinations,
        };
        for destination_name in destinations {
            if let Some(d_module) = modules.get(destination_name) {
                if let Module::Conjunction { .. } = d_module {
                    conjunction_inputs
                        .entry(destination_name.clone())
                        .or_insert(vec![])
                        .push(name.clone());
                }
            }
        }
    }
    for (name, inputs) in conjunction_inputs {
        let module = modules.get_mut(&name).unwrap();
        if let Module::Conjunction(m) = module {
            for i in inputs {
                m.last_seen.insert(i, false);
            }
        }
    }

    return modules;
}

fn result(input: &str) -> u64 {
    let mut modules = read_modules(input);

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        let mut pulses = VecDeque::<Pulse>::new();
        low_pulses += 1;
        pulses.push_back(Pulse {
            is_high: false,
            location: "broadcaster".to_string(),
            prev_location: "button".to_string(),
        });

        while !pulses.is_empty() {
            let pulse = pulses.pop_front().unwrap();
            let next_pulses = pulse.process_pulse(&mut modules);

            let next_high_pulses = next_pulses.iter().filter(|p| p.is_high).count() as u64;
            high_pulses += next_high_pulses;
            low_pulses += next_pulses.len() as u64 - next_high_pulses;

            pulses.extend(next_pulses);
        }
    }

    return low_pulses * high_pulses;
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The result after 1000 button presses is: {}", result(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example_1() {
        let input = std::fs::read_to_string("../test_input_1.txt").unwrap();
        assert_eq!(result(&input), 32000000);
    }

    #[test]
    fn it_solves_example_2() {
        let input = std::fs::read_to_string("../test_input_2.txt").unwrap();
        assert_eq!(result(&input), 11687500);
    }
}
