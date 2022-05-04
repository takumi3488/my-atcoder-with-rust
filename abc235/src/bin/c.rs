use proconio::{fastout, input, marker::Usize1};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        qn: usize,
        a: [usize; n],
        xk: [(usize,Usize1); qn]
    }
    let mut hm: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        if let Some(x) = hm.get_mut(&a[i]) {
            (*x).push(i + 1);
        } else {
            hm.insert(a[i], vec![i + 1]);
        }
    }
    for q in xk {
        let len = if let Some(x) = hm.get(&q.0) {
            x.len()
        } else {
            0
        };
        if len > q.1 {
            println!("{}", hm.get(&q.0).unwrap()[q.1]);
        } else {
            println!("-1");
        }
    }
}
