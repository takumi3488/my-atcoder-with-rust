use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize}
    let mut g = vec![vec![]; n];
    for _ in 1..n {
        input! {x:Usize1,y:Usize1}
        g[x].push(x)
    }
}
