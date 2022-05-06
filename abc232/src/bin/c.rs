use itertools::Itertools;
use proconio::{input, fastout, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        cd: [(Usize1, Usize1); m]
    }
    for p in (0..n).permutations(n) {
        if ab.iter().all(|&(a, b)| cd.contains(&(p[a], p[b])) || cd.contains(&(p[b], p[a]))) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
