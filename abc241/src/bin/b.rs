use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m]
    }
    let mut hm = HashMap::new();
    for ai in a {
        let aai = hm.entry(ai).or_insert(0);
        *aai += 1;
    }
    for bi in b {
        let op = hm.get(&bi);
        if op.is_some() && op.unwrap() > &0 {
            hm.insert(bi, op.unwrap() - 1);
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes")
}
