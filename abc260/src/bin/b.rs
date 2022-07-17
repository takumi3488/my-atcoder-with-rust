use std::collections::HashSet;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        mut y: usize,
        mut z: usize,
        a: [usize; n],
        b: [usize; n]
    }
    let mut aa: Vec<(usize, usize)> = a.iter().enumerate().map(|(i, &v)| (i + 1, v)).collect_vec();
    let mut bb: Vec<(usize, usize)> = b.iter().enumerate().map(|(i, &v)| (i + 1, v)).collect_vec();
    let mut cc: Vec<(usize, usize)> = vec![];
    for i in 0..n {
        cc.push((i + 1, aa[i].1 + bb[i].1));
    }
    aa.sort_by(|(_, x), (_, y)| y.cmp(x));
    bb.sort_by(|(_, x), (_, y)| y.cmp(x));
    cc.sort_by(|(_, x), (_, y)| y.cmp(x));
    let mut res: HashSet<usize> = HashSet::new();
    for &ai in &aa[..x] {
        res.insert(ai.0);
    }
    for &bi in &bb {
        if y == 0 {
            break;
        }
        if !res.contains(&bi.0) {
            y -= 1;
            res.insert(bi.0);
        }
    }
    for &ci in &cc {
        if z == 0 {
            break;
        }
        if !res.contains(&ci.0) {
            z -= 1;
            res.insert(ci.0);
        }
    }
    for i in res.iter().sorted() {
        println!("{}", i);
    }
}
