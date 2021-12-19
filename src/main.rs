#![allow(dead_code, unused_variables)]

use std::slice;

#[cfg(test)]

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Elem {
    Open,
    Num(usize),
    Close,
}

use crate::Elem::*;

fn main() {
    part1().unwrap();
}

fn part1() -> Option<()> {
    let num = add_list(get_input());
    println!("Magnitude: {}", magnitude(num));
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


fn split(starting: &Vec<Elem>) -> Option<Vec<Elem>> {
    let mut did_one = false;
    let after_split: Vec<Elem> = starting.into_iter().flat_map(|e| -> Vec<Elem> {
        if let Num(v) = e {
            if !did_one && *v > 9 {
                did_one = true;
                let rounded_up = (v + (2 - 1)) / 2;
                return vec![Open, Num(v / 2), Num(rounded_up), Close];
            }
        }
        vec![*e]
    }).collect();

    match did_one {
        false => None,
        true => Some(after_split),
    }
}

fn add(a: &Vec<Elem>, b: &Vec<Elem>) -> Vec<Elem> {
    let mut result: Vec<Elem> =
        [slice::from_ref(&Open), a.as_slice(), b.as_slice(), slice::from_ref(&Close)].concat();
    loop {
        result = match explode(&result) {
            None => match split(&result) {
                None => break,
                Some(new_result) => new_result,
            }
            Some(new_result) => new_result,
        }
    }
    result
}

#[test]
fn test_add() {
    let a = "[[[[4,3],4],4],[7,[[8,4],9]]]";
    let b = "[1,1]";
    let result = add(&parse_number(a), &parse_number(b));
    assert!(equiv(result, parse_number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")));
}

#[test]
fn test_example() {
    let sam = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
    assert!(equiv(add_list(sam), parse_number("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")));
}

#[test]
fn test_example2() {
    let sam = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
    assert!(equiv(add_list(sam),
                  parse_number("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")));
}

fn magnitude(num: Vec<Elem>) -> usize {
    if num.len() == 1 {
        return match num.get(0) {
            Some(Num(n)) => *n,
            _ => panic!("Bad number"),
        };
    }

    let num_iter = num.into_iter();
    let take_one = |skip_count: usize| -> Vec<Elem>  {
        let mut depth = 0;
        let mut did_one = false;
        num_iter.clone().skip(skip_count).take_while(|e: &Elem| -> bool {
            let old_depth = depth;
            depth = match e {
                Open => depth + 1,
                Close => depth - 1,
                _ => depth,
            };
            let now_stop = old_depth == 0 && did_one;
            did_one = true;
            !now_stop
        }).collect()
    };
    let left = take_one(1);
    let right = take_one(1 + left.len());
    return 3 * magnitude(left) + 2 * magnitude(right);
}

#[test]
fn test_magnitude() {
    assert_eq!(magnitude(parse_number("[[1,2],[[3,4],5]]")), 143);
    assert_eq!(magnitude(parse_number("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")), 4140);
}

fn add_list(input: &str) -> Vec<Elem> {
    input.lines().map(parse_number).fold(
        None,
        |prev: Option<Vec<Elem>>, num: Vec<Elem>| -> Option<Vec<Elem>> {
            match prev {
                Some(prev_num) => Some(add(&prev_num, &num)),
                None => Some(num),
            }
        }).unwrap()
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
            let foo: Vec<Elem> = starting.into_iter().take(p).rev().map(|e| *e).collect();
            let mut head: Vec<Elem> = starting.into_iter().take(p - 1).rev().map(|e| -> Elem {
                if let Num(_) = e {
                    if !did_left {
                        did_left = true;
                        return plus_elems(*e, left);
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
                        return plus_elems(*p, right);
                    }
                }
                *p
            }).collect();

            Some([head.as_slice(), slice::from_ref(&Num(0)), tail.as_slice()].concat())
        }
    }
}

fn plus_elems(a: Elem, b: Elem) -> Elem {
    match (a, b) {
        (Num(na), Num(nb)) => Num(na + nb),
        _ => panic!("Can't plus")
    }
}

fn get_input() -> &'static str {
    return "[[1,[8,[5,8]]],[[4,4],[8,[8,8]]]]
[[[3,[2,3]],[[8,0],2]],[0,[[8,1],[7,0]]]]
[4,[[0,3],[[6,6],[3,8]]]]
[[[7,[6,4]],[[0,6],[2,0]]],[[[5,6],[0,4]],[[8,1],[9,1]]]]
[[[6,3],[[6,9],4]],[[1,[4,2]],[[0,0],1]]]
[[2,0],[3,[0,8]]]
[[0,[5,5]],[[4,2],[3,[6,4]]]]
[[[[9,9],[8,5]],[7,4]],[[6,9],[8,[0,8]]]]
[[[[7,1],[2,9]],[[9,3],0]],[3,[[0,6],[7,6]]]]
[[[[3,7],[7,1]],[[5,8],[0,1]]],3]
[[[[4,6],[6,2]],[[9,1],7]],[[9,1],[8,0]]]
[[[[2,7],0],[[9,4],[2,6]]],[0,[[7,4],[0,3]]]]
[[5,[[0,2],[8,8]]],[[[4,1],9],3]]
[[[7,1],[[3,7],[3,4]]],[[[0,7],[1,6]],1]]
[[[6,5],[[1,8],[8,8]]],[[4,5],[3,7]]]
[[[1,[3,3]],[[3,2],[5,7]]],[[8,[9,3]],[[5,3],4]]]
[[[4,[2,7]],9],[9,[[5,6],4]]]
[[[9,1],3],[[1,2],9]]
[[[[0,0],[2,3]],[[7,8],[1,5]]],[[[8,6],7],[[8,3],9]]]
[6,[[5,[0,8]],1]]
[4,[[[3,0],[2,0]],[[7,2],[1,4]]]]
[[[[4,3],[4,1]],8],[[[9,4],[1,9]],[4,[0,6]]]]
[4,[5,6]]
[[[0,[6,1]],[[6,1],3]],[[0,[7,8]],[1,0]]]
[[5,[[8,7],8]],8]
[[5,[[5,2],0]],[[1,[4,7]],[[0,9],[2,3]]]]
[[7,[2,2]],[[6,3],[5,8]]]
[[[0,9],5],[1,[[5,7],1]]]
[[8,[3,[0,3]]],[[[2,2],2],[[8,8],[8,9]]]]
[[6,[[3,2],[2,6]]],[5,1]]
[[[[9,8],[6,8]],[0,7]],7]
[[[7,2],[[6,3],4]],2]
[[[5,2],[[1,6],[8,3]]],[6,5]]
[[5,2],[0,5]]
[[[[4,5],5],[[4,6],[1,2]]],[[[3,6],[4,9]],[1,9]]]
[[1,[4,1]],[[9,[5,5]],[[9,0],[5,7]]]]
[[[[8,9],[7,7]],2],[8,1]]
[[[8,1],[8,[9,5]]],3]
[[[2,[3,9]],[[5,4],[7,9]]],[9,8]]
[8,[[2,[0,9]],[[5,0],4]]]
[[[6,[4,8]],[0,6]],[[8,[1,8]],1]]
[[6,[[1,0],[6,2]]],[[9,[3,7]],[5,[4,0]]]]
[[8,[0,[9,1]]],8]
[7,[4,[7,2]]]
[[1,[[5,7],[5,4]]],[[5,[8,0]],[1,6]]]
[[[[0,6],[6,2]],3],[[[9,3],7],[7,[1,2]]]]
[[[6,[4,9]],8],[6,5]]
[[[0,[1,9]],[[1,9],[3,9]]],[[[3,4],[7,5]],3]]
[[[[9,3],5],[[0,5],[2,7]]],9]
[[[6,[7,5]],5],[1,[[7,0],[3,4]]]]
[[[2,1],[[1,3],[1,5]]],[4,[9,[7,9]]]]
[[[[7,9],4],[[8,8],7]],[[[3,5],2],[[4,4],[6,5]]]]
[[1,1],[1,1]]
[[8,[0,2]],8]
[[[2,[2,1]],[[1,7],[1,2]]],[[1,6],5]]
[6,[0,[[1,0],[0,9]]]]
[6,[[2,[8,0]],[8,[8,8]]]]
[4,[[3,[0,3]],4]]
[[[5,3],3],[[0,[7,6]],[2,[5,8]]]]
[[[[8,1],[4,1]],[[5,8],[4,8]]],[[[1,7],[7,2]],[0,[2,7]]]]
[[[2,[3,5]],3],5]
[[7,[[9,5],[8,2]]],[[[1,8],8],5]]
[[3,5],[[4,[9,3]],5]]
[[[[4,6],2],[2,2]],[0,[0,4]]]
[[[[5,8],[6,6]],[2,0]],[[[2,3],9],[[4,5],2]]]
[[[[1,9],3],[[3,4],6]],[[3,6],[6,[0,7]]]]
[[[0,[5,5]],[2,6]],[[[7,4],4],2]]
[0,[[8,[6,2]],[5,[1,5]]]]
[[[[5,5],[9,6]],[[5,2],2]],[[4,7],[[5,5],[1,6]]]]
[[4,7],[[[1,8],[9,6]],[2,3]]]
[5,[5,4]]
[[[[2,1],[7,0]],[5,[7,8]]],[6,[3,1]]]
[[[3,1],[[2,4],6]],[[[1,8],[2,1]],[[1,7],4]]]
[[[5,[3,3]],6],[[[0,0],9],[1,[7,4]]]]
[[[6,5],[[7,3],4]],[[9,[0,3]],[3,[6,0]]]]
[[[3,4],7],[8,[[1,7],[9,9]]]]
[[[[2,1],6],[2,6]],[[[8,1],[6,2]],[9,0]]]
[[8,4],[5,2]]
[[4,[[4,5],9]],[[3,[5,2]],[4,2]]]
[[[8,8],[[8,0],[5,3]]],4]
[[1,8],[0,2]]
[[[[7,2],[9,0]],[[9,2],[1,2]]],[[[4,0],3],0]]
[[[[1,2],[1,8]],[[4,3],[8,6]]],[[[5,1],8],[8,1]]]
[[[[5,3],[7,2]],7],[[6,[7,9]],[[3,8],[9,4]]]]
[[[[3,1],[2,5]],6],[[[3,2],[8,8]],[4,6]]]
[9,[[3,[2,3]],6]]
[[[[4,0],[5,6]],[5,4]],[[[9,0],[1,8]],[5,[3,6]]]]
[[[[9,5],[9,4]],[[5,7],5]],[[[1,4],7],[6,1]]]
[[2,[6,[8,2]]],[7,[1,[3,3]]]]
[[[9,1],[0,[6,3]]],[[5,[1,5]],[7,[1,0]]]]
[1,6]
[[0,[2,[8,9]]],[[[4,5],[5,4]],1]]
[[[1,[4,1]],8],[[2,[7,0]],[7,[9,9]]]]
[[[[5,7],[3,5]],[[6,6],2]],[2,[8,[9,0]]]]
[6,[[[3,9],8],[[4,3],[6,1]]]]
[[[[6,7],[7,6]],[2,8]],[[9,[4,1]],6]]
[[[[4,5],[4,5]],[[0,6],5]],[[[6,5],[7,0]],1]]
[[[[6,7],9],[[5,5],[6,6]]],[[7,1],[[8,2],[3,1]]]]
[[[9,6],7],[[[1,8],8],[1,7]]]
[[5,2],[[1,9],[2,2]]]";
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


