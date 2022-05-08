use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: (isize,isize),
        mut b: (isize,isize),
        s: [Chars; n]
    }
    a = (a.0 - 1, a.1 - 1);
    b = (b.0 - 1, b.1 - 1);
    let mut que = vec![a];
    let mut visited = HashSet::new();
    visited.insert(a);
    let mut res = 1;
    while que.len() > 0 {
        for _ in 0..que.len() {
            let p = que.pop().unwrap();
            visited.insert(p);
            for (i, &dir) in (&[(-1, -1), (-1, 1), (1, -1), (1, 1)]).iter().enumerate() {
                let x = p.0 + i as isize * dir.0;
                let y = p.1 + i as isize * dir.1;
                if 0 <= x
                    && x < n as isize
                    && 0 <= y
                    && y < n as isize
                    && s[x as usize][y as usize] == '.'
                {
                    if b == (x, y) {
                        println!("{}", res);
                        return;
                    }
                    que.push((x, y));
                }
            }
        }
        res += 1;
    }
    println!("-1");
}
