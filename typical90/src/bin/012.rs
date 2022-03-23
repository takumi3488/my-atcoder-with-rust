use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize
    }
    let pos = |x: usize, y: usize| (x - 1) * w + y - 1;
    let mut red = vec![vec![false; w]; h];
    let mut uf = UnionFind::<usize>::new(h * w);
    for _ in 0..q {
        input! {
            t: usize
        }
        if t == 1 {
            input! {
                x: usize,
                y: usize
            }
            red[x-1][y-1] = true;
            for &(dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                if (dx == -1 && x == 1) || (dy == -1 && y == 1) {
                    continue;
                }
                let p = x + (dx as usize);
                let q = y + (dy as usize);
                if p - 1 < h && q - 1 < w && red[p][q] {
                    uf.union(pos(x, y), pos(p, q));
                }
            }
        } else {
            input!(xa: Usize1, ya: Usize1, xb: Usize1, yb: Usize1);
            if red[xa][ya] && uf.equiv(pos(xa, ya), pos(xb, yb)) {
                println!("Yes");
            } else {
                println!("No");
            };
        }
    }
}
