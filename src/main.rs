#![allow(dead_code, unused_variables)]

use std::slice;
use crate::Elem::{Open, Num, Close};

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Elem {
    Open,
    Num(usize),
    Close,
}

fn main() {
    part1().unwrap();
}

fn part1() -> Option<()> {
    Some(())
}

fn parse_number(s: &str) -> Vec<Elem> {
    s.chars().filter_map(|c: char| -> Option<Elem> {
        match c {
            c if c == '[' => {
                Some(Open)
            }
            c if c == ']' => {
                Some(Close)
            }
            c if c == ',' => {
                None
            }
            _ => Some(Num(c.to_digit(10).unwrap() as usize))
        }
    }).collect()
}

fn explode(starting: &Vec<Elem>) -> Option<Vec<Elem>> {
    let mut depth = 0;
    let maybe_pos = starting.into_iter().enumerate().find_map(|(p, e): (usize, &Elem)| -> Option<usize> {
        match e {
            Open => {
                depth += 1;
                None
            }
            Close => {
                depth -= 1;
                None
            }
            Num(_) => if depth > 4 { Some(p) } else { None },
        }
    });
    match maybe_pos {
        None => None,
        Some(p) => {
            let (left, right): (Elem, Elem) = (*starting.get(p)?, *starting.get(p + 1)?);
            let mut did_left = false;
            let foo:Vec<Elem> = starting.into_iter().take(p).rev().map(|e| *e).collect();
            let mut head: Vec<Elem> = starting.into_iter().take(p - 1).rev().map(|e| -> Elem {
                if let Num(_) = e {
                    if !did_left {
                        did_left = true;
                        return plus(*e, left);
                    }
                }
                *e
            }).collect();
            head.reverse();

            let mut did_right = false;
            let tail: Vec<Elem> = starting.into_iter().skip(p + 3).map(|p| -> Elem {
                if let Num(_) = p {
                    if !did_right {
                        did_right = true;
                        return plus(*p, right);
                    }
                }
                *p
            }).collect();

            Some([head.as_slice(), slice::from_ref(&Num(0)), tail.as_slice()].concat())
        }
    }
}

fn plus(a: Elem, b: Elem) -> Elem {
    match (a, b) {
        (Num(na), Num(nb)) => Num(na + nb),
        _ => panic!("Can't plus")
    }
}

fn get_input() -> &'static str {
    return "[1,1]
[2,2]
[3,3]
[4,4]";
}

#[test]
fn test_explode() {
    assert!(equiv(explode(&parse_number("[[[[[9,8],1],2],3],4]")).unwrap(),
                  parse_number("[[[[0,9],2],3],4]")));
}

#[test]
fn test_explode2() {
    assert!(equiv(explode(&parse_number("[7,[6,[5,[4,[3,2]]]]]")).unwrap(),
                  parse_number("[7,[6,[5,[7,0]]]]")));
}

#[test]
fn test_explode3() {
    assert!(equiv(explode(&parse_number("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")).unwrap(),
                  parse_number("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")));

    assert!(equiv(explode(&parse_number("[[6,[5,[4,[3,2]]]],1]")).unwrap(),
                  parse_number("[[6,[5,[7,0]]],3]")));

    assert!(equiv(explode(&parse_number("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")).unwrap(),
                  parse_number("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")));
}

#[test]
fn test_parse() {
    let parsed = parse_number("[[[[[9,8],1],2],3],4]");
    assert_eq!(parsed.iter().filter(|e| **e == Open).count(),
               parsed.iter().filter(|e| **e == Close).count());
}

fn equiv(a: Vec<Elem>, b: Vec<Elem>) -> bool {
    if a.len() != b.len() { false } else {
        a.iter().zip(b).filter(|(ea, eb)| **ea != *eb).count() == 0
    }
}


