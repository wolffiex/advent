#![allow(dead_code)]

use std::collections::HashSet;


type Grid = Vec<Vec<u32>>;

#[derive(PartialEq, Eq, Hash)]
struct Point(usize, usize);

fn main() {
    fn convert_line(line: &str) -> Vec<u32> {
        return line.chars().map(|c| -> u32{ c.to_digit(10).unwrap() }).collect();
    }

    let split = get_input().split("\n");
    let initial: Grid = split.into_iter().map(convert_line).collect();
    let next = step(initial);
}

fn step(current: Grid) -> Grid {
    //increment
    let mut next: Grid = current.into_iter().map(|row| -> Vec<u32>{
        row.into_iter().map(|n| -> u32{ n + 1 }).collect()
    }).collect();

    let mut flashed: HashSet<Point> = HashSet::new();

    loop {
        let mut pass_flashed: HashSet<Point> = HashSet::new();
        for y in 0..next.len() {
            let row = next.get(y).unwrap();
            for x in 0..row.len() {
                let val = row.get(x).unwrap();
            }
        }

        if pass_flashed.len() == 0 { break; }
    }
    return next;
}

fn get_input() -> &'static str {
    return "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
}