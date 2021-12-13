#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug)]
struct Candidate {
    did_small: bool,
    path: Vec<&'static str>,
}

fn main() {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in get_input().split("\n") {
        let mut nodes = line.split("-");
        let a = nodes.next().unwrap();
        let b = nodes.next().unwrap();
        map.entry(a).or_default().push(b);
        map.entry(b).or_default().push(a);
    }
    let mut stack: Vec<Candidate> = vec! {Candidate { did_small: false, path: vec! {"start"} }};
    let mut paths_through: Vec<Vec<&str>> = Vec::new();
    loop {
        let candidate = match stack.pop() {
            Some(p) => p,
            None => break,
        };
        let next_nodes = map.get(candidate.path.last().unwrap()).unwrap();
        for node in next_nodes {
            if let Some(next_candidate) = try_candidate(&candidate, node) {
                // println!("path: {:?}", new_path);
                if node.contains("end") {
                    paths_through.push(next_candidate.path);
                } else {
                    stack.push(next_candidate);
                }
                // println!("stack: {:?}", stack);
            }
        }
    }
    println!("HO: {:?}", paths_through.len());
}

fn try_candidate(candidate: &Candidate, node: &'static str) -> Option<Candidate> {
    if node.contains("start") {
        return None;
    } else {
        let mut did_small = candidate.did_small;
        if node.chars().next().unwrap().is_lowercase() {
            if candidate.path.contains(&node) {
                if did_small { return None; }
                did_small = true;
            }
        }
        let mut path = candidate.path.clone();
        path.push(node);
        return Some(Candidate {
            did_small,
            path,
        });
    }
}

fn xget_input() -> &'static str {
    return "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
}

fn get_input() -> &'static str {
    return "vn-DD
qm-DD
MV-xy
end-xy
KG-end
end-kw
qm-xy
start-vn
MV-vn
vn-ko
lj-KG
DD-xy
lj-kh
lj-MV
ko-MV
kw-qm
qm-MV
lj-kw
VH-lj
ko-qm
ko-start
MV-start
DD-ko";
}
