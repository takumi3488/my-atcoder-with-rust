use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {q: usize}
    let mut v: VecDeque<(usize, usize)> = VecDeque::new();
    for _ in 0..q {
        input! {n: usize}
        if n == 1 {
            input! {
                x: usize,
                c: usize
            }
            v.push_back((x, c));
        } else {
            input! {mut c: usize}
            let mut res = 0;
            while c > 0 {
                let mut f = v.pop_front().unwrap();
                if f.1 > c {
                    f.1 -= c;
                    res += c * f.0;
                    c = 0;
                    v.push_front(f);
                } else {
                    c -= f.1;
                    res += f.0 * f.1;
                }
            }
            println!("{}", res);
        }
    }
}
