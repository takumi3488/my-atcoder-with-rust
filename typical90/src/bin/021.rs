use petgraph::{algo::kosaraju_scc, graph::Graph};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _: usize,
        m: usize,
        edges: [(u32, u32); m]
    }
    let g = Graph::<(), ()>::from_edges(&edges);
    let v = kosaraju_scc(&g);
    println!(
        "{}",
        v.iter().map(|x| x.len() * (x.len() - 1) / 2).sum::<usize>()
    )
}
