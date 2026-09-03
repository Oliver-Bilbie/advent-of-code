use std::collections::HashMap;
use wasm_bindgen::prelude::*;

type Pair = (char, char);

fn read_template(input: &str) -> HashMap<Pair, u64> {
    let mut counts = HashMap::new();
    let chars = input.lines().nth(0).unwrap().chars().collect::<Vec<char>>();
    for i in 1..chars.len() {
        counts
            .entry((chars[i - 1], chars[i]))
            .and_modify(|ct| *ct += 1)
            .or_insert(1);
    }
    counts
}

fn read_rules(input: &str) -> HashMap<Pair, char> {
    let mut rules = HashMap::new();
    for line in input.lines().skip(2) {
        let chars = line.chars().collect::<Vec<char>>();
        rules.insert((chars[0], chars[1]), chars[6]);
    }
    rules
}

fn score(counts: &HashMap<Pair, u64>, input: &str) -> u64 {
    let mut element_ct: [u64; 26] = [0; 26];
    let char_idx = |c: char| c as usize - 'A' as usize;

    // To count elements, we count only the first item in each pair to avoid duplication.
    // We must therefore also add the final element manually.
    for (p, ct) in counts {
        element_ct[char_idx(p.0)] += ct;
    }
    element_ct[char_idx(input.lines().nth(0).unwrap().chars().last().unwrap())] += 1;

    let max_ct = element_ct.iter().max().unwrap();
    let min_ct = element_ct
        .iter()
        .fold(u64::MAX, |acc, x| if *x > 0 { acc.min(*x) } else { acc });
    max_ct - min_ct
}

fn result(input: &str) -> u64 {
    let rules = read_rules(input);
    let mut counts = read_template(input);

    for _ in 0..40 {
        let mut next_counts = HashMap::<Pair, u64>::new();
        for (p, ct) in &counts {
            let next_pairs = match rules.get(&p) {
                Some(r) => {
                    vec![(p.0, *r), (*r, p.1)]
                }
                None => vec![*p],
            };
            for next_p in next_pairs {
                next_counts
                    .entry(next_p)
                    .and_modify(|next_ct| *next_ct += ct)
                    .or_insert(*ct);
            }
        }
        counts = next_counts;
    }

    score(&counts, input)
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The output is: {}", result(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 2188189693529);
    }
}
