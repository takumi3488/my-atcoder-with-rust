use proconio::{input,fastout, marker::Usize1};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        q: usize,
        xy: [(Usize1,Usize1);q]
    }
    let mut av = vec![HashSet::new()];
    av[0].insert(a[0]);
    let mut bv = vec![HashSet::new()];
    bv[0].insert(b[0]);
    for i in 1..n {
        av.push(av[i-1].clone());
        av[i].insert(a[i]);
        bv.push(bv[i-1].clone());
        bv[i].insert(b[i]);
    }
    for xy in xy {
        if av[xy.0].symmetric_difference(&bv[xy.1]).count() > 0 {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
