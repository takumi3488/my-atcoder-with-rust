use proconio::input;
use std::collections::HashMap;
use std::cmp::{min, max};
use std::usize::{MAX, MIN};

fn main() {
    input! {
        n: usize,
        p: [(usize, usize); n],
        s: String
    }
    let mut hm: HashMap<usize, (usize, usize)> = HashMap::new();
    for (i, si) in s.chars().into_iter().enumerate() {
        let r = *hm.entry(p[i].1).or_insert((MIN, MAX));
        if si.to_string() == String::from("R") {
            hm.insert(p[i].1, (r.0, min(p[i].0, r.1)));
        } else if si.to_string() == String::from("L") {
            hm.insert(p[i].1, (max(p[i].0, r.0), r.1));
        }
        let h = hm.get(&p[i].1).unwrap();
        if h.0 > h.1 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
