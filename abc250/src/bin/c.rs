use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q]
    }
    let mut xp = HashMap::new();
    let mut px = HashMap::new();
    xp.insert(n, n);
    xp.insert(1, 1);
    px.insert(n, n);
    px.insert(1, 1);
    for x in x {
        let v = *xp.entry(x).or_insert(x);
        if v == n {
            let lv = *px.entry(v - 1).or_insert(v - 1);
            xp.insert(lv, v);
            xp.insert(x, v - 1);
            px.insert(v - 1, x);
            px.insert(v, lv);
            continue;
        }
        let rv = *px.entry(v + 1).or_insert(v + 1);
        xp.insert(rv, v);
        xp.insert(x, v + 1);
        px.insert(v + 1, x);
        px.insert(v, rv);
    }
    for i in 1..=n {
        print!("{} ", px.get(&i).unwrap_or(&i));
    }
    println!()
}
