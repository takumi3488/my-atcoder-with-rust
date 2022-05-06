use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1);m]
    }
    let mut v = vec![0; n + 1];
    let mut uf = UnionFind::new(n);
    for ab in ab {
        v[ab.0] += 1;
        v[ab.1] += 1;
        if v[ab.0] > 2 || v[ab.1] > 2 || !uf.union(ab.0, ab.1) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
