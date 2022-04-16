use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [(f64,f64); n]
    }
    if k == 1 {
        println!("Infinity");
        return;
    }
    let mut res = 0;
    for c in p.iter().combinations(2) {
        let a = ((c[1].1 - c[0].1) / (c[1].0 - c[0].0)).abs();
        let mut ki = 0;
        for pi in p.clone() {
            let ai = ((pi.1 - c[0].1) / (pi.0 - c[0].0)).abs();
            if a == ai {
                ki += 1;
            }
        }
        if ki >= k {
            res += 1;
        }
    }
    println!("{}", res)
}
