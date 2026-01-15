use std::collections::HashMap;
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

    fn result(&self, left: u16, right: u16) -> bool {
        match self {
            Self::GreaterThan => left > right,
            Self::LessThan => left < right,
        }
    }
}

struct Rule {
    category: Category,
    comparator: Comparator,
    value: u16,
    target: String,
}

struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl Part {
    fn from_str(input: &str) -> Self {
        let values: Vec<u16> = input[1..input.len() - 1]
            .split(',')
            .map(|s| s[2..].parse().unwrap())
            .collect();
        Self {
            x: values[0],
            m: values[1],
            a: values[2],
            s: values[3],
        }
    }

    fn get_category(&self, category: &Category) -> u16 {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn sum(&self) -> u64 {
        self.x as u64 + self.m as u64 + self.a as u64 + self.s as u64
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

fn process_part(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut workflow = workflows
        .get("in")
        .expect("input should contain an 'in' workflow");

    loop {
        let mut rule_matched = false;

        for rule in &workflow.rules {
            if rule
                .comparator
                .result(part.get_category(&rule.category), rule.value)
            {
                if rule.target == "A" {
                    return true;
                }
                if rule.target == "R" {
                    return false;
                }
                workflow = workflows.get(&rule.target).unwrap();
                rule_matched = true;
                break;
            }
        }

        if !rule_matched {
            if workflow.target == "A" {
                return true;
            }
            if workflow.target == "R" {
                return false;
            }
            workflow = workflows.get(&workflow.target).unwrap();
        }
    }
}

fn result(input: &str) -> u64 {
    let workflows = read_workflows(input);
    let parts: Vec<Part> = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| Part::from_str(l))
        .filter(|p| process_part(p, &workflows))
        .collect();
    return parts.iter().map(|p| p.sum()).sum();
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The sum of rating numbers is: {}", result(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 19114);
    }
}
