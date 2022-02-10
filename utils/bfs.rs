use std::collections::VecDeque;

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
