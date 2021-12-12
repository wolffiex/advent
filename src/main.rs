#![allow(dead_code)]

use std::collections::HashMap;


fn main() {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in get_input().split("\n") {
        let mut nodes = line.split("-");
        let a = nodes.next().unwrap();
        let b = nodes.next().unwrap();
        map.entry(a).or_default().push(b);
        map.entry(b).or_default().push(a);
    }
    let mut stack:Vec<Vec<&str>> = vec!{vec!{"start"}};
    let mut paths_through:Vec<Vec<&str>> = Vec::new();
    loop {
        let path = match stack.pop() {
            Some(p) => p,
            None => break,
        };
        let path_end = path.last().unwrap();
        println!("want to get : {:?}", path_end);
        let next_nodes = map.get(path_end).unwrap();
        for node in next_nodes {
            let is_big = node.chars().next().unwrap().is_uppercase();
            if is_big || !path.contains(node) {
                let mut new_path = path.clone();
                new_path.push(node);
                println!("path: {:?}", new_path);
                if node.contains("end") {
                    paths_through.push(new_path);
                } else {
                    stack.push(new_path);
                }
                println!("stack: {:?}", stack);
            }
        }
    }
    println!("HO: {:?}", paths_through.len());
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
DD-ko"
}
