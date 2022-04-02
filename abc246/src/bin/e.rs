use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: isize,
        a: (isize,isize),
        b: (isize,isize),
        s: [Chars; n]
    }
    let mut g = vec![];
    let dirs = vec![(-1, -1), (-1, 1), (1, -1), (1, 1)];
    for i in 0..n {
        for j in 0..n {
            for dir in dirs.iter() {
                let mut p = (i, j);
                while 0 <= i && i < n && 0 <= j && j < n {
                    p.0 += dir.0;
                    p.1 += dir.1;
                    if &s[i as usize][j as usize].to_string() == "#" {
                        break;
                    }
                    g.push(vec![tog(i, j, n), tog(p.0, p.1, n)]);
                }
            }
        }
    }
    let mut g = vec![]
}

fn tog(x: isize, y: isize, n: isize) -> isize {
    x * n + y
}

fn bfs(g: &Vec<Vec<usize>>, d: &mut Vec<usize>, s: usize) {
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
