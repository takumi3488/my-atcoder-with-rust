use petgraph::{algo::dijkstra, graph::UnGraph};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
        m: usize,
        edges: [(u32,u32,usize);m]
    }
    let g = UnGraph::<(), usize>::from_edges(edges);
    let from_1 = dijkstra(&g, 1.into(), None, |x| *x.weight());
    let from_n = dijkstra(&g, n.into(), None, |x| *x.weight());
    for i in 1..=n {
        println!("{}", from_1[&i.into()] + from_n[&i.into()]);
    }
}
