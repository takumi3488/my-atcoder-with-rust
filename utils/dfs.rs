// g: グラフ, dist,u: 頂点uからの各頂点の距離, 
// p,d: 再帰のための便宜上の引数(初期値はそれぞれusize::MAX,0)
fn dfs(g: &Vec<Vec<usize>>, dist: &mut Vec<usize>, u: &usize, p: &usize, d: usize) {
    dist[*u] = d;
    for v in g[*u].iter() {
        if v == p {
            continue;
        }
        dfs(g, dist, v, u, d + 1);
    }
}
