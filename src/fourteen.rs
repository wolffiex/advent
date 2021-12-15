#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    part2().unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Pair(char, char);

fn part1() -> Option<()> {
    let (mut sequence, rule_map) = process_input(get_input())?;
    for s in 0..10 {
        sequence = step(sequence, &rule_map);
        println!("Step {}:{}", s, sequence.len());
    }
    let counts = count_chars(&sequence);
    let (max_k, max_v) = counts.iter().max_by_key(|&(_, v)| v)?;
    let (min_k, min_v) = counts.iter().min_by_key(|&(_, v)| v)?;
    println!("{}:{}", max_k, max_v);
    println!("{}:{}", min_k, min_v);
    println!("result:{}", max_v - min_v);
    return Some(());
}

enum VisitState { Unvisited, DidLeft, DidRight }

fn part2() -> Option<()> {
    let (initial, rule_map) = process_input(get_input())?;

    let mut freq_map: HashMap<char, usize> = HashMap::new();
    let mut freq_count = |c: char| {
        *freq_map.entry(c).or_insert(0) += 1;
    };

    let mut expand = |original_pair: Pair| {
        let mut stack: Vec<(Pair, VisitState)> = vec![(original_pair, VisitState::Unvisited)];
        let mut i: usize = 0;
        loop {
            if i %10000000 == 0 {
                println!("l: {}", i);
            }
            i = i + 1;
            if stack.is_empty(){ break; }
            let is_max_depth = stack.len() >= 30;
            let (pair, visit_state) = stack.pop().unwrap();
            let e = *rule_map.get(&pair).unwrap();
            match visit_state {
                VisitState::Unvisited => {
                    stack.push((pair, VisitState::DidLeft));
                    if !is_max_depth {
                        stack.push((Pair(pair.0, e), VisitState::Unvisited));
                    }
                }
                VisitState::DidLeft => {
                    //print!("{}", e);
                    freq_count(e);
                    stack.push((pair, VisitState::DidRight));
                    if !is_max_depth {
                        stack.push((Pair(e, pair.1), VisitState::Unvisited));
                    }
                }
                VisitState::DidRight => {}
            }
        }
    };

    //println!("");
    //println!("");
    //print!("{}", initial.chars().next()?);
    for i in 0..initial.len() - 1 {
        let a = initial.chars().nth(i)?;
        let b = initial.chars().nth(i + 1)?;
        expand(Pair(a, b));
        //print!("{}", b);
    }
    //println!("");
    //println!("");

    let mut kill_me = freq_map.clone();
    for c in initial.chars() {
        *kill_me.entry(c).or_insert(0) += 1;
    }
    for (c, v) in kill_me.into_iter() {
        println!("{}:{}", c, v);
    }
    Some(())
}

fn count_chars(sequence: &str) -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = HashMap::new();
    for c in sequence.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    return map;
}

fn step(sequence: String, rules: &HashMap<Pair, char>) -> String {
    let mut last_char = None;
    return sequence.chars().map(|c| -> String {
        let old_last = last_char;
        last_char = Some(c);
        match old_last {
            None => c.into(),
            Some(last) => {
                let new_char = rules.get(&Pair(last, c)).unwrap();
                format!("{}{}", new_char, c)
            }
        }
    }).collect();
}

fn process_input(input: &str) -> Option<(String, HashMap<Pair, char>)> {
    let (sequence, rules) = input.split_once("\n\n")?;
    return Some((sequence.into(), rules.lines().filter_map(
        |line: &str| -> Option<(Pair, char)> {
            Some({
                let (a, b) = line.split_once(" -> ")?;
                (head_pair(a)?, b.chars().next()?)
            })
        }
    ).collect()));
}

fn head_pair(s: &str) -> Option<Pair> {
    let mut it = s.chars();
    Some(Pair(it.next()?, it.next()?))
}

fn xget_input() -> &'static str {
    return "SVCHKVFKCSHVFNBKKPOC

NC -> H
PK -> V
SO -> C
PH -> F
FP -> N
PN -> B
NP -> V
NK -> S
FV -> P
SB -> S
VN -> F
SC -> H
OB -> F
ON -> O
HN -> V
HC -> F
SN -> K
CB -> H
OP -> K
HP -> H
KS -> S
BC -> S
VB -> V
FC -> B
BH -> C
HH -> O
KH -> S
VF -> F
PF -> P
VV -> F
PP -> V
BO -> H
BF -> B
PS -> K
FO -> O
KF -> O
FN -> H
CK -> B
VP -> V
HK -> F
OV -> P
CS -> V
FF -> P
OH -> N
VS -> H
VO -> O
CP -> O
KC -> V
KV -> P
BK -> B
VK -> S
NF -> V
OO -> V
FH -> H
CN -> O
SP -> B
KN -> V
OF -> H
NV -> H
FK -> B
PV -> N
NB -> B
KK -> P
VH -> P
CC -> B
HV -> V
OC -> H
PO -> V
NO -> O
BP -> C
NH -> H
BN -> O
BV -> S
CV -> B
HS -> O
NN -> S
NS -> P
KB -> F
CO -> H
HO -> P
PB -> B
BS -> P
SH -> H
FS -> V
SF -> O
OK -> F
KP -> S
BB -> C
PC -> B
OS -> C
SV -> N
SK -> K
KO -> C
SS -> V
CF -> C
HB -> K
VC -> B
CH -> P
HF -> K
FB -> V";
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
