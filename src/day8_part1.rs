use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn dist2(a: Point, b: Point) -> i128 {
    let dx = (a.x - b.x) as i128;
    let dy = (a.y - b.y) as i128;
    let dz = (a.z - b.z) as i128;
    dx * dx + dy * dy + dz * dz
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    d2: i128,
    a: usize,
    b: usize,
}

impl Eq for Edge {}
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.d2 == other.d2 && self.a == other.a && self.b == other.b
    }
}
impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.d2
            .cmp(&other.d2)
            .then_with(|| self.a.cmp(&other.a))
            .then_with(|| self.b.cmp(&other.b))
    }
}
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<u64>,
}
impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }
    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }
        // union by size
        if self.size[ra] < self.size[rb] {
            self.parent[ra] = rb;
            self.size[rb] += self.size[ra];
        } else {
            self.parent[rb] = ra;
            self.size[ra] += self.size[rb];
        }
    }
    fn component_sizes(mut self) -> Vec<u64> {
        let n = self.parent.len();
        for i in 0..n {
            let _ = self.find(i);
        }
        let mut seen = vec![false; n];
        let mut sizes = Vec::new();
        for i in 0..n {
            let r = self.parent[i];
            if !seen[r] {
                seen[r] = true;
                sizes.push(self.size[r]);
            }
        }
        sizes
    }
}

pub fn run() {
    let input = include_str!("../inputs/day8input.txt");

    let points: Vec<Point> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let mut it = l.split(',');
            let x: i64 = it.next().unwrap().trim().parse().unwrap();
            let y: i64 = it.next().unwrap().trim().parse().unwrap();
            let z: i64 = it.next().unwrap().trim().parse().unwrap();
            Point { x, y, z }
        })
        .collect();

    let n = points.len();
    let k: usize = 1000;

    let mut heap: BinaryHeap<Edge> = BinaryHeap::new();

    for i in 0..n {
        for j in (i + 1)..n {
            let d2 = dist2(points[i], points[j]);
            let e = Edge { d2, a: i, b: j };

            if heap.len() < k {
                heap.push(e);
            } else if let Some(top) = heap.peek() {
                if e.d2 < top.d2 {
                    heap.pop();
                    heap.push(e);
                }
            }
        }
    }

    let mut dsu = DSU::new(n);
    while let Some(e) = heap.pop() {
        dsu.union(e.a, e.b);
    }

    let mut sizes = dsu.component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    let product: u128 = (sizes.get(0).copied().unwrap_or(1) as u128)
        * (sizes.get(1).copied().unwrap_or(1) as u128)
        * (sizes.get(2).copied().unwrap_or(1) as u128);

    println!("{}", product);
}
