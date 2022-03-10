use proconio::{input,marker::Usize1};

fn main() {
    input! { n: usize }
    let mut g = vec![vec![]; n];
    for _ in 1..n {
        input! {
            a: Usize1,
            b: Usize1
        }
        g[a].push(b);
        g[b].push(a);
    }
    let mut u = 0;
    let mut dist = vec![0; n];
    dfs(&g, &mut dist, &u, &std::usize::MAX, 0);
    let max_dist = dist.iter().max().unwrap();
    for v in 0..n {
        if dist[v] == *max_dist {
            u = v;
            break;
        }
    }
    dfs(&g, &mut dist, &u, &std::usize::MAX, 0);
    println!("{}", dist.iter().max().unwrap() + 1);
}

fn dfs(g: &Vec<Vec<usize>>, dist: &mut Vec<usize>, u: &usize, p: &usize, d: usize) {
    dist[*u] = d;
    for v in g[*u].iter() {
        if v == p {
            continue;
        }
        dfs(g, dist, v, u, d + 1);
    }
}
