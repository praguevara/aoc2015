use core::panic;
use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

type Replacements<'a> = HashMap<&'a [u8], Vec<&'a [u8]>>;

fn parse_input(input: &str) -> (Replacements, &[u8]) {
    let (replacements_str, target_str) = input.split_once("\n\n").unwrap();
    let replacements: HashMap<&[u8], Vec<&[u8]>> = replacements_str
        .lines()
        .map(|l| l.split_once(" => ").unwrap())
        .map(|(k, v)| (k.as_bytes(), v.as_bytes()))
        .fold(
            HashMap::new(),
            |mut acc: HashMap<&[u8], Vec<&[u8]>>, (k, v)| {
                let entry = acc.entry(k);
                match entry {
                    Entry::Occupied(mut e) => {
                        e.get_mut().push(v);
                    }
                    Entry::Vacant(e) => {
                        e.insert(vec![v]);
                    }
                }
                acc
            },
        );
    (replacements, target_str.trim_end().as_bytes())
}

fn replace_at(target: &[u8], from: &[u8], to: &[u8], idx: usize) -> Box<[u8]> {
    [&target[0..idx], &to, &target[idx + from.len()..]]
        .concat()
        .into_boxed_slice()
}

#[test]
fn test_replace_at() {
    let target = "hello guys :)".as_bytes();
    let from = "guys".as_bytes();
    let to = "people".as_bytes();
    let idx = 6;
    assert_eq!(
        replace_at(target, from, to, idx),
        "hello people :)".as_bytes().to_vec().into_boxed_slice(),
    );
}

fn apply_replacement(target: &[u8], from: &[u8], to: &[u8]) -> Vec<Box<[u8]>> {
    let mut results = vec![];
    for i in 0..target.len() {
        if target.get(i..i + from.len()) == Some(from) {
            results.push(replace_at(target, from, to, i));
        }
    }
    results
}

fn part_one(replacements: &Replacements, target: &[u8]) -> usize {
    let mut distinct: HashSet<Box<[u8]>> = HashSet::new();
    let target_bytes = target;
    for (k, v) in replacements {
        for vv in v {
            let res = apply_replacement(target_bytes, k, vv);
            distinct.extend(res);
        }
    }
    distinct.len()
}

fn undo_replacements(target: &[u8], from: &[u8], to: &[u8]) -> Vec<Box<[u8]>> {
    apply_replacement(target, to, from)
}

fn part_two(replacements: &Replacements, target: &[u8], from: &[u8]) -> usize {
    let mut queue = VecDeque::from(vec![(from.to_vec().into_boxed_slice(), 0)]);
    let mut visited: HashSet<Box<[u8]>> = HashSet::new();
    while let Some((from, cost)) = queue.pop_back() {
        if visited.contains(&from) {
            continue;
        }

        if *from == *target {
            return cost;
        }

        let mut children = vec![];
        for (k, v) in replacements {
            for vv in v {
                let res = undo_replacements(&from, k, vv);
                children.extend(res);
            }
        }
        for child in children {
            queue.push_back((child, cost + 1));
        }

        visited.insert(from);
    }

    panic!("Solution not found!");
}

#[test]
fn test_part_two() {
    let input = include_str!("../input.txt");
    let (replacements, _target) = parse_input(input);
    let res = part_two(&replacements, "e".as_bytes(), "OTiTiTiMg".as_bytes());
    assert_eq!(4, res);
}

fn main() {
    let input = include_str!("../input.txt");
    let (replacements, target) = parse_input(input);

    println!("First star: {:?}", part_one(&replacements, target));
    println!(
        "Second star: {:?}",
        part_two(&replacements, "e".as_bytes(), target)
    );
}
