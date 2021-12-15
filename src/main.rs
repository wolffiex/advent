#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    part1().unwrap();
}

fn part1() -> Option<()> {
    println!("hi");
    let map: HashMap<Point, usize> = HashMap::new();
    let mut bests: HashMap<Point, Vec<Point>> = HashMap::new();
    let mut paths = vec![vec![Point { x: 0, y: 0 }]];
    loop {
        if paths.is_empty() { break; }
        let path = paths.pop()?;
        let last = path.last()?;
        let candidates: Vec<Point> = vec![(-1, 0), (0, -1), (1, 0), (0, 1)]
            .into_iter()
            .filter_map(|m| from_point(m, last))
            .collect();

        for p in candidates {
            let mut candidate_path = path.clone();
            candidate_path.push(p);
            let risk = calc_risk(&candidate_path, &map)?;
            let is_better = match bests.get(&p) {
                None => true,
                Some(last_path) => {
                    let last_risk = calc_risk(last_path, &map)?;
                    last_risk > risk
                }
            };
            if is_better {
                bests.insert(p, candidate_path.clone());
                paths.push(candidate_path)
            }
        }
    }
    let p = Point { x: 0, y: 0 };
    println!("Best: {:?}", p);

    Some(())
}

fn from_point((dx, dy): (isize, isize), p: &Point) -> Option<Point> {
    fn safe_add(d: isize, n: usize) -> Option<usize> {
        if d < 0 { n.checked_sub(d.abs() as usize) } else { n.checked_add(d as usize) }
    }
    Some(Point {
        x: safe_add(dx, p.x)?,
        y: safe_add(dy, p.y)?,
    })
}

fn calc_risk(path: &Vec<Point>, map: &HashMap<Point, usize>) -> Option<usize> {
    path.into_iter().fold(
        Some(0),
        |v, p| Some(v? + map.get(&p)?))
}

fn get_input() -> &'static str {
    return "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
}