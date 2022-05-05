use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n]
    }
    let mut res = 0usize;
    let mut hm = std::collections::HashMap::new();
    let mut s = 0;
    for a in a {
        *hm.entry(s).or_insert(0) += 1;
        s += a;
        res += *hm.get(&(s - k)).unwrap_or(&0);
    }

    println!("{}", res);
}
