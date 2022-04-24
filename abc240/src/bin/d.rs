use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut v: VecDeque<(usize, usize)> = VecDeque::new();
    let mut c = 0;
    for ai in a {
        c += 1;
        match v.back() {
            Some(x) => {
                if x.0 == ai {
                    let b = v.pop_back().unwrap();
                    if ai == b.1 + 1 {
                        c -= ai;
                    } else {
                        v.push_back((ai, b.1 + 1));
                    }
                } else {
                    v.push_back((ai, 1));
                }
            }
            None => {
                v.push_back((ai, 1));
            }
        }
        println!("{}", c);
    }
}
