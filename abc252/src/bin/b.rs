use proconio::{fastout, input, marker::Usize1};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
        b: [Usize1; k]
    }
    let _max = *a.iter().max().unwrap();
    let mut hs = HashSet::new();
    for (i, &a) in (&a).iter().enumerate() {
        if a == _max {
            hs.insert(i);
        }
    }
    for b in &b {
        if hs.get(b).is_some() {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
