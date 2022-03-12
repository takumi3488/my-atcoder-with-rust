use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut hm = HashMap::new();
    let mut c = 0;
    let mut d = 0;
    for i in 0..n {
        if a[i] == b[i] {
            c += 1;
        } else {
            if let Some(&n) = hm.get(&a[i]) {
                d += 1;
            } else {
                hm.insert(&a[i], 1);
            }
            if let Some(&n) = hm.get(&b[i]) {
                d += 1;
            } else {
                hm.insert(&b[i], 1);
            }
        }
    }
    println!("{}", c);
    println!("{}", d);
}
