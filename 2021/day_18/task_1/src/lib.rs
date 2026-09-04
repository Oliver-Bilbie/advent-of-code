use wasm_bindgen::prelude::*;

type Pair = (Item, Item);

#[derive(Clone, Debug)]
enum Item {
    Value(u8),
    Child(Box<Pair>),
}

fn read_snailfish_number(line: &str) -> Option<Pair> {
    let mut depth = 0;
    let mut center = None;
    let mut end = None;

    for (i, c) in line.chars().enumerate() {
        match c {
            '[' => {
                depth += 1;
            }
            ']' => {
                depth -= 1;
                if depth == 0 {
                    end = Some(i);
                    break;
                }
            }
            ',' => {
                if depth == 1 {
                    center = Some(i);
                }
            }
            _ => {}
        }
    }

    if center.is_none() || end.is_none() {
        return None;
    }
    let mut parts: Vec<Option<Item>> = [
        &line[1..center.unwrap()],
        &line[center.unwrap() + 1..end.unwrap()],
    ]
    .into_iter()
    .map(|s: &str| -> Option<Item> {
        if s.chars().nth(0).unwrap() == '[' {
            if let Some(child) = read_snailfish_number(s) {
                return Some(Item::Child(Box::new(child)));
            }
        }
        if let Ok(v) = s.parse::<u8>() {
            return Some(Item::Value(v));
        }
        None
    })
    .collect();

    if parts.iter().any(|v| v.is_none()) || parts.len() != 2 {
        return None;
    }
    let r = parts.pop().unwrap().unwrap();
    let l = parts.pop().unwrap().unwrap();
    Some((l, r))
}

fn add(left: Pair, right: Pair) -> Pair {
    let mut n = (Item::Child(Box::new(left)), Item::Child(Box::new(right)));
    loop {
        if explode(&mut n) {
            continue;
        }
        if split(&mut n) {
            continue;
        }
        break;
    }
    n
}

fn explode(number: &mut Pair) -> bool {
    explode_pair(number, 0).is_some()
}

fn explode_pair(pair: &mut Pair, depth: u8) -> Option<(Option<u8>, Option<u8>)> {
    if let Some((left, right)) = explode_item(&mut pair.0, depth + 1) {
        if let Some(right) = right {
            add_leftmost(&mut pair.1, right);
        }
        return Some((left, None));
    }
    if let Some((left, right)) = explode_item(&mut pair.1, depth + 1) {
        if let Some(left) = left {
            add_rightmost(&mut pair.0, left);
        }
        return Some((None, right));
    }
    None
}

fn explode_item(item: &mut Item, depth: u8) -> Option<(Option<u8>, Option<u8>)> {
    if depth >= 4 {
        let exploded = match item {
            Item::Child(pair) => match (&pair.0, &pair.1) {
                (Item::Value(l), Item::Value(r)) => Some((*l, *r)),
                _ => None,
            },
            Item::Value(_) => None,
        };
        if let Some((l, r)) = exploded {
            *item = Item::Value(0);
            return Some((Some(l), Some(r)));
        }
    }
    match item {
        Item::Child(pair) => explode_pair(pair, depth),
        Item::Value(_) => None,
    }
}

fn add_leftmost(item: &mut Item, v: u8) {
    match item {
        Item::Value(n) => *n += v,
        Item::Child(pair) => add_leftmost(&mut pair.0, v),
    }
}

fn add_rightmost(item: &mut Item, v: u8) {
    match item {
        Item::Value(n) => *n += v,
        Item::Child(pair) => add_rightmost(&mut pair.1, v),
    }
}

fn split(number: &mut Pair) -> bool {
    split_item(&mut number.0) || split_item(&mut number.1)
}

fn split_item(item: &mut Item) -> bool {
    match item {
        Item::Value(v) if *v > 9 => {
            let v = *v;
            *item = Item::Child(Box::new((Item::Value(v / 2), Item::Value(v.div_ceil(2)))));
            true
        }
        Item::Child(pair) => split(pair),
        Item::Value(_) => false,
    }
}

fn magnitude(pair: &Pair) -> u64 {
    3 * item_magnitude(&pair.0) + 2 * item_magnitude(&pair.1)
}

fn item_magnitude(item: &Item) -> u64 {
    match item {
        Item::Value(v) => u64::from(*v),
        Item::Child(pair) => magnitude(pair),
    }
}

fn result(input: &str) -> u64 {
    let numbers: Vec<Pair> = input
        .lines()
        .map(|line| read_snailfish_number(line).expect("invalid snailfish number"))
        .collect();
    let first = numbers[0].clone();
    let total = numbers
        .into_iter()
        .skip(1)
        .fold(first, |acc, n| add(acc, n));
    magnitude(&total)
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The magnitude of the final sum is: {}", result(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 3488);
    }
}
