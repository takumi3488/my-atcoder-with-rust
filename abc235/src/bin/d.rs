use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        a: usize,
        n: usize
    }
    let mut m = 1;
    while m <= n {
        m *= 10;
    }
    let mut d = vec![-1; m];
    d[1] = 0;
    let mut v: VecDeque<usize> = VecDeque::new();
    v.push_back(1);
    while v.len() > 0 {
        let c = v.pop_front().unwrap();
        let dc = d[c];
        let op1 = a * c;
        if op1 < m && d[op1] == -1 {
            d[op1] = dc + 1;
            v.push_back(op1);
        }
        if c >= 10 && c % 10 > 0 {
            let op2 = (c % 10) * 10usize.pow((c.to_string().len() - 1) as u32) + c / 10;
            if op2 < m && d[op2] == -1 {
                d[op2] = dc + 1;
                v.push_back(op2);
            }
        }
    }
    println!("{}", d[n]);
}
