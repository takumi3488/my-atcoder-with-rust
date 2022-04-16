use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        qn: usize,
        q: [(usize,usize,usize); qn]
    }
    let mut v = vec![vec![]; n + 1];
    for i in 0..n {
        v[a[i]].push(i + 1);
    }
    for qi in q {
        let mut d = 1;
        let l = v[qi.2].binary_search(&qi.0).unwrap_or_else(|x| x);
        let r = v[qi.2].binary_search(&qi.1).unwrap_or_else(|x| {
            d -= 1;
            x
        });
        println!("{}", d + r - l);
    }
}
