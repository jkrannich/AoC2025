#[derive(Clone)]
struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
            root
        }
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        // union by size
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        self.components -= 1;
        true
    }
}

fn parse_points(input: &str) -> Vec<(i64, i64, i64)> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let mut it = line.split(',');
            let x = it.next().unwrap().trim().parse::<i64>().unwrap();
            let y = it.next().unwrap().trim().parse::<i64>().unwrap();
            let z = it.next().unwrap().trim().parse::<i64>().unwrap();
            (x, y, z)
        })
        .collect()
}

pub fn run() {
    let input = include_str!("../inputs/day8input.txt");
    let pts = parse_points(input);
    let n = pts.len();

    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    edges.reserve(n * (n - 1) / 2);

    for i in 0..n {
        let (xi, yi, zi) = pts[i];
        for j in (i + 1)..n {
            let (xj, yj, zj) = pts[j];
            let dx = xi - xj;
            let dy = yi - yj;
            let dz = zi - zj;
            let dist2 = dx * dx + dy * dy + dz * dz;
            edges.push((dist2, i, j));
        }
    }

    edges.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut dsu = Dsu::new(n);

    for (_d2, i, j) in edges {
        if dsu.union(i, j) {
            if dsu.components == 1 {
                let ans = pts[i].0 * pts[j].0;
                println!("{ans}");
                return;
            }
        }
    }

    panic!("Never reached a single connected circuit");
}
