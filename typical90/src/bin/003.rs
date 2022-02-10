use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn main() {
    input! {n:usize}
    let mut g = vec![vec![]; n];
    for _ in 1..n {
        input! {x:Usize1,y:Usize1}
        g[x].push(y);
        g[y].push(x);
    }
    let mut d = vec![1 << 30; n];
    bfs(&g, &mut d, 0usize);
    let mut u: usize = 0;
    let m = d.iter().max().unwrap();
    for i in 0..n {
        if &d[i] == m {
            u = i
        }
    }
    let mut d = vec![1 << 30; n];
    bfs(&g, &mut d, u);
    println!("{}", 1 + d.iter().max().unwrap());
}

fn bfs(g: &Vec<Vec<usize>>, d: &mut Vec<isize>, s: usize) {
    let mut q: VecDeque<usize> = VecDeque::new();
    d[s] = 0;
    q.push_back(s);
    while q.len() > 0 {
        let i = q.pop_front().unwrap();
        for j in g[i].iter() {
            if d[*j] == 1 << 30 {
                q.push_back(*j);
                d[*j] = d[i] + 1;
            }
        }
    }
}
