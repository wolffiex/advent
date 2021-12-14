#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    part1().unwrap();
}

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
struct Pair(char, char);

fn part1() -> Option<()> {
    let (mut sequence, rule_map) = process_input(get_input())?;
    for _ in 0..10 {
        sequence = step(sequence, &rule_map)?;
    }
    println!("{}", sequence.len());
    return Some(());
}

fn step(sequence: String, rules: &HashMap<Pair, char>) -> Option<String> {
    let mut it = sequence.chars();
    let last_char = it.next()?;
    let next_step = it.map(|c| -> String {
        let new_char = rules.get(&Pair(last_char, c)).unwrap();
        format!("{}{}", last_char, new_char)
    }).collect::<String>();
    return Some(format!("{}{}", sequence.chars().last()?, next_step));
}

fn process_input(input: &str) -> Option<(String, HashMap<Pair, char>)> {
    let (sequence, rules) = input.split_once("\n\n")?;
    return Some((sequence.into(), rules.lines().filter_map(
        |line: &str| -> Option<(Pair, _)> {
            Some({
                let (a, b) = line.split_once(" -> ")?;
                let pair = head_pair(a);
                (pair?, b.chars().next()?)
            })
        }
    ).collect()));
}

fn head_pair(s: &str) -> Option<Pair> {
    let mut it = s.chars();
    Some(Pair(it.next()?, it.next()?))
}

fn get_input() -> &'static str {
    return "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
}