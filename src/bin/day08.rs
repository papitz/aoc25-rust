use std::collections::{HashMap, HashSet};

use rust_aoc::read_lines;

const DAY: u8 = 8;

fn main() {
    let ex_lines = read_lines(DAY, true);

    println!("Part 1 result: {}", solve_part_1(&ex_lines, 10));
    println!("Part 2 result: {}", solve_part_2(&ex_lines));
    // //
    let lines = read_lines(DAY, false);
    println!("Part 1 result: {}", solve_part_1(&lines, 1000));
    println!("Part 2 result: {}", solve_part_2(&lines));
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct JumperBox(i64, i64, i64);

impl JumperBox {
    fn new(x: i64, y: i64, z: i64) -> Self {
        JumperBox(x, y, z)
    }

    fn parse_str(str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let value_strings: Vec<&str> = str.split(',').collect();
        let [x, y, z] = value_strings
            .iter()
            .map(|str| str.parse::<i64>())
            .collect::<Result<Vec<i64>, _>>()?
            .try_into()
            .map_err(|_| "Input array did not have exactly 3 elements")?;
        Ok(JumperBox::new(x, y, z))
    }

    fn distance_to(&self, other: &Self) -> f64 {
        let dx = other.0 - self.0;
        let dy = other.1 - self.1;
        let dz = other.2 - self.2;
        f64::sqrt(dx.pow(2) as f64 + dy.pow(2) as f64 + dz.pow(2) as f64)
    }
}

#[derive(Debug)]
pub struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        let size = vec![1; n]; // Initialize size to 1 for each element
        let count = n;

        for i in 0..n {
            parent[i] = i; // Each element is its own parent initially
        }

        DisjointSet { parent, size, count }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            let root = self.find(self.parent[i]);
            self.parent[i] = root; // Path compression
            root
        }
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i == root_j {
            return false; // Already in the same set
        }

        // Attach smaller tree to larger tree
        if self.size[root_i] < self.size[root_j] {
            self.parent[root_i] = root_j;
            self.size[root_j] += self.size[root_i];
        } else {
            self.parent[root_j] = root_i;
            self.size[root_i] += self.size[root_j];
        }
        self.count -= 1;
        true
    }
}

#[derive(Debug)]
// struct Connection<'a> {
struct Connection {
    dist: f64,
    // b1: &'a JumperBox,
    // b2: &'a JumperBox,
    b1: usize,
    b2: usize,
}

// fn gather_connections(boxes: &'_ [JumperBox]) -> Vec<Connection<'_>> {
fn gather_connections(boxes: &[JumperBox]) -> Vec<Connection> {
    gather_connections_rec(boxes, 0)
}

fn gather_connections_rec(boxes: &[JumperBox], index: usize) -> Vec<Connection> {
    if boxes.is_empty() {
        return vec![];
    }
    let mut connections: Vec<Connection> = Vec::new();
    let (current_box, rest) = boxes.split_first().unwrap();
    for (o_index, other) in rest.iter().enumerate() {
        connections.push(Connection {
            dist: current_box.distance_to(other),
            b1: index,
            b2: index + o_index + 1,
        })
    }
    connections.extend(gather_connections_rec(rest, index + 1));
    connections
}

fn solve_part_1(lines: &Vec<String>, no_of_pairs: usize) -> usize {
    let mut boxes: Vec<JumperBox> = Vec::new();
    for line in lines {
        boxes.push(JumperBox::parse_str(line).ok().unwrap());
    }
    let mut connections: Vec<Connection> = gather_connections(&boxes);
    connections.sort_unstable_by(|a, b| a.dist.total_cmp(&b.dist));
    let best_cons: Vec<&Connection> = connections.iter().take(no_of_pairs).collect();
    let mut box_ids_in_best_cons: HashSet<usize> = HashSet::new();
    for conn in &best_cons {
        box_ids_in_best_cons.insert(conn.b1);
        box_ids_in_best_cons.insert(conn.b2);
    }
    let new_ids: HashMap<usize, usize> = Vec::from_iter(box_ids_in_best_cons)
        .iter()
        .enumerate()
        .map(|(new_id, old_id)| (*old_id, new_id))
        .collect();

    let mut dsu = DisjointSet::new(new_ids.len());
    for conn in &best_cons {
        // dbg!(new_ids.get(&conn.b1));
        println!("Union of {} and {}", conn.b1, conn.b2);
        let b1 = *new_ids.get(&conn.b1).unwrap();
        let b2 = *new_ids.get(&conn.b2).unwrap();
        println!("{b1}{b2}");
        dsu.union(b1, b2);
    }
    let mut size_ary = dsu.size;
    size_ary.sort();
    size_ary.iter().rev().take(3).product()
}

fn solve_part_2(lines: &Vec<String>) -> i64 {
    let mut boxes: Vec<JumperBox> = Vec::new();
    for line in lines {
        boxes.push(JumperBox::parse_str(line).ok().unwrap());
    }
    let mut connections: Vec<Connection> = gather_connections(&boxes);
    connections.sort_unstable_by(|a, b| a.dist.total_cmp(&b.dist));

    let mut dsu = DisjointSet::new(boxes.len());
    for conn in &connections {
        println!("Union of {} and {}", conn.b1, conn.b2);
        let b1 = conn.b1;
        let b2 = conn.b2;
        dsu.union(b1, b2);
        println!("{}", dsu.count);
        if dsu.count <= 1 {
            let x1 = boxes[b1].0;
            let x2 = boxes[b2].0;
            return x1 * x2;
        }
    }
    0
}
