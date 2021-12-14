#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    part1().unwrap();
}

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
struct Pair(char, char);

fn part1() -> Option<()> {
    let (initial, rules) = get_input().split_once("\n\n")?;
    let rule_map: HashMap<Pair, char> = rules.lines().filter_map(
        |line: &str| -> Option<(Pair, _)> {
            Some({
                let (a, b) = line.split_once(" -> ")?;
                let pair = head_pair(a.into());
                (pair?, b.chars().next()?)
            })
        }
    ).collect();
    let mut sequence:String = initial.into();
    for i in 0..10 {
        sequence = step(sequence, &rule_map);
    }
    return Some(());
}

fn head_pair(s:String) -> Option<Pair>{
    if let [a, b, ..] = &s[0..2] {
        return Some(Pair(a,b));
    }
    None
}
fn step(sequence:String, rules:&HashMap<Pair, char>) -> String {
    return  String::from("dldkfjd");
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