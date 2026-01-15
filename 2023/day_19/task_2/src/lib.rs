use std::collections::{HashMap, VecDeque};
use wasm_bindgen::prelude::*;

enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn from_char(input: char) -> Self {
        match input {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!("{} is not a valid category", input),
        }
    }
}

enum Comparator {
    GreaterThan,
    LessThan,
}

impl Comparator {
    fn from_char(input: char) -> Self {
        match input {
            '>' => Self::GreaterThan,
            '<' => Self::LessThan,
            _ => panic!("{} is not a valid comparator", input),
        }
    }
}

struct Rule {
    category: Category,
    comparator: Comparator,
    value: u16,
    target: String,
}

struct SplitResult<T> {
    passed: Option<T>,
    failed: Option<T>,
}

#[derive(Clone, Debug)]
struct Range {
    begin: u16,
    end: u16,
}

impl Range {
    fn len(&self) -> u64 {
        self.end as u64 - self.begin as u64 + 1
    }

    fn split_by_rule(self, rule: &Rule) -> SplitResult<Self> {
        if rule.value > self.end {
            return match rule.comparator {
                Comparator::GreaterThan => SplitResult {
                    passed: None,
                    failed: Some(self),
                },
                Comparator::LessThan => SplitResult {
                    passed: Some(self),
                    failed: None,
                },
            };
        }

        if rule.value < self.begin {
            return match rule.comparator {
                Comparator::GreaterThan => SplitResult {
                    passed: Some(self),
                    failed: None,
                },
                Comparator::LessThan => SplitResult {
                    passed: None,
                    failed: Some(self),
                },
            };
        }

        match rule.comparator {
            Comparator::GreaterThan => SplitResult {
                passed: Some(Self {
                    begin: rule.value + 1,
                    end: self.end,
                }),
                failed: Some(Self {
                    begin: self.begin,
                    end: rule.value,
                }),
            },
            Comparator::LessThan => SplitResult {
                passed: Some(Self {
                    begin: self.begin,
                    end: rule.value - 1,
                }),
                failed: Some(Self {
                    begin: rule.value,
                    end: self.end,
                }),
            },
        }
    }
}

#[derive(Debug)]
struct PartRanges {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRanges {
    fn size(&self) -> u64 {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }

    fn split_by_rule(self, rule: &Rule) -> SplitResult<Self> {
        match rule.category {
            Category::X => {
                let split_result = self.x.split_by_rule(rule);
                SplitResult {
                    passed: match split_result.passed {
                        Some(rng) => Some(Self {
                            x: rng,
                            m: self.m.clone(),
                            a: self.a.clone(),
                            s: self.s.clone(),
                        }),
                        None => None,
                    },
                    failed: match split_result.failed {
                        Some(rng) => Some(Self {
                            x: rng,
                            m: self.m,
                            a: self.a,
                            s: self.s,
                        }),
                        None => None,
                    },
                }
            }
            Category::M => {
                let split_result = self.m.split_by_rule(rule);
                SplitResult {
                    passed: match split_result.passed {
                        Some(rng) => Some(Self {
                            x: self.x.clone(),
                            m: rng,
                            a: self.a.clone(),
                            s: self.s.clone(),
                        }),
                        None => None,
                    },
                    failed: match split_result.failed {
                        Some(rng) => Some(Self {
                            x: self.x,
                            m: rng,
                            a: self.a,
                            s: self.s,
                        }),
                        None => None,
                    },
                }
            }
            Category::A => {
                let split_result = self.a.split_by_rule(rule);
                SplitResult {
                    passed: match split_result.passed {
                        Some(rng) => Some(Self {
                            x: self.x.clone(),
                            m: self.m.clone(),
                            a: rng,
                            s: self.s.clone(),
                        }),
                        None => None,
                    },
                    failed: match split_result.failed {
                        Some(rng) => Some(Self {
                            x: self.x,
                            m: self.m,
                            a: rng,
                            s: self.s,
                        }),
                        None => None,
                    },
                }
            }
            Category::S => {
                let split_result = self.s.split_by_rule(rule);
                SplitResult {
                    passed: match split_result.passed {
                        Some(rng) => Some(Self {
                            x: self.x.clone(),
                            m: self.m.clone(),
                            a: self.a.clone(),
                            s: rng,
                        }),
                        None => None,
                    },
                    failed: match split_result.failed {
                        Some(rng) => Some(Self {
                            x: self.x,
                            m: self.m,
                            a: self.a,
                            s: rng,
                        }),
                        None => None,
                    },
                }
            }
        }
    }
}

struct Workflow {
    rules: Vec<Rule>,
    target: String,
}

fn read_workflows(input: &str) -> HashMap<String, Workflow> {
    let mut workflows = HashMap::<String, Workflow>::new();

    for line in input.lines().take_while(|l| !l.is_empty()) {
        let (name, rules_str) = line[..line.len() - 1].split_once('{').unwrap();
        let rules_items: Vec<&str> = rules_str.split(',').collect();
        let rules: Vec<Rule> = rules_items
            .iter()
            .take(rules_items.len().saturating_sub(1))
            .map(|s| Rule {
                category: Category::from_char(s.chars().nth(0).unwrap()),
                comparator: Comparator::from_char(s.chars().nth(1).unwrap()),
                value: s
                    .chars()
                    .skip(2)
                    .take_while(|c| *c != ':')
                    .collect::<String>()
                    .parse()
                    .unwrap(),
                target: s.chars().skip_while(|c| *c != ':').skip(1).collect(),
            })
            .collect();
        let target = rules_items.last().unwrap().to_string();
        workflows.insert(name.to_string(), Workflow { rules, target });
    }

    return workflows;
}

fn result(input: &str) -> u64 {
    let workflows = read_workflows(input);
    let mut total: u64 = 0;

    let mut part_ranges = VecDeque::<(PartRanges, &Workflow)>::new();
    let max_range = Range {
        begin: 1,
        end: 4000,
    };
    part_ranges.push_back((
        PartRanges {
            x: max_range.clone(),
            m: max_range.clone(),
            a: max_range.clone(),
            s: max_range.clone(),
        },
        workflows
            .get("in")
            .expect("input should contain an 'in' workflow"),
    ));

    while !part_ranges.is_empty() {
        let (start_range, workflow) = part_ranges.pop_front().unwrap();
        let mut current_range: Option<PartRanges> = Some(start_range);

        for rule in &workflow.rules {
            let rule_result = match current_range {
                Some(rng) => rng.split_by_rule(rule),
                None => {
                    break;
                }
            };

            if let Some(passed_range) = rule_result.passed {
                if rule.target == "A" {
                    total += passed_range.size();
                } else if rule.target != "R" {
                    part_ranges.push_back((passed_range, workflows.get(&rule.target).unwrap()));
                }
            }

            current_range = rule_result.failed;
        }

        if let Some(rng) = current_range {
            if workflow.target == "A" {
                total += rng.size();
            } else if workflow.target != "R" {
                part_ranges.push_back((rng, workflows.get(&workflow.target).unwrap()));
            }
        }
    }

    return total;
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!(
        "There are {} distinct combinations of ratings will be accepted by the Elves' workflows",
        result(input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 167409079868000);
    }
}
