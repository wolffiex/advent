#![allow(dead_code)]

use std::collections::HashSet;


type Grid = Vec<Vec<u32>>;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    fn convert_line(line: &str) -> Vec<u32> {
        return line.chars().map(|c| -> u32{ c.to_digit(10).unwrap() }).collect();
    }

    let split = get_input().split("\n");
    let mut grid: Grid = split.into_iter().map(convert_line).collect();
    let size = grid.len() * grid[0].len();
    // print_grid(&grid);
    let mut total = 0;
    let mut step_count = 0;
    loop {
        let (next, flashes) = step(&grid);
        step_count = step_count + 1;
        total = total + flashes.len();
        grid = next;
        if flashes.len() == size {
            println!("step {}: {}", step_count, flashes.len());
            print_grid(&grid);
            break;
        }
    }
    println!("pp : {}", total);
}

fn step(current: &Grid) -> (Grid, HashSet<Point>) {
    //increment
    let mut next: Grid = current.into_iter().map(|row| -> Vec<u32>{
        row.into_iter().map(|n| -> u32{ n + 1 }).collect()
    }).collect();

    let width = next.get(0).unwrap().len();
    let mut all_flashed: HashSet<Point> = HashSet::new();

    loop {
        let mut pass_flashed: HashSet<Point> = HashSet::new();
        for y in 0..next.len() {
            for x in 0..width {
                let p = Point { x, y };
                let v = next[y][x];
                if v > 9u32 && !all_flashed.contains(&p) {
                    pass_flashed.insert(p);
                }
            }
        }

        if pass_flashed.len() > 0 {
            for pf in pass_flashed {
                for dx in -1i8..=1 {
                    for dy in -1i8..=1 {
                        if let Some(p) = translate_point(pf, dx, dy) {
                            if let Some(row) = next.get_mut(p.y) {
                                if let Some(v) = row.get_mut(p.x) {
                                    *v = *v + 1;
                                }
                            }
                        }
                    }
                }
                all_flashed.insert(pf);
            }
        } else {
            for p in &all_flashed {
                *next.get_mut(p.y).unwrap().get_mut(p.x).unwrap() = 0;
            }
            break;
        }
    }
    return (next, all_flashed);
}

fn translate_point(p: Point, dx: i8, dy: i8) -> Option<Point> {
    fn positive_sum(u: usize, du: i8) -> Option<usize> {
        let nu = u as isize + du as isize;
        return if nu < 0 { None } else { Some(nu as usize) };
    }
    return Some(Point { x: positive_sum(p.x, dx)?, y: positive_sum(p.y, dy)? });
}

fn print_grid(grid: &Grid) {
    for row in grid {
        println!("{:?}", row);
    }
}

fn get_input() -> &'static str {
    return "6318185732
1122687135
5173237676
8754362612
5718474666
8443654137
1247634346
1446514585
6717288267
1727871228";
}

fn xget_input() -> &'static str {
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