use proconio::{input,fastout, marker::Usize1};
use std::{cmp::Reverse, collections::BinaryHeap};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1,Usize1,usize); m]
    }
    let mut edge = vec![vec![];m];
    for a in abc {
        edge[a.0].push((a.1,a.2));
        edge[a.1].push((a.0,a.2));
    }
    let d = Dijkstra::new(n, edge, 0);
    for p in d.get_path(n-1) {
        print!("{} ", p+1);
    }
    println!();
}
