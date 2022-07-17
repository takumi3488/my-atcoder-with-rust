use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n]
    }
    let mut v: Vec<(usize, Vec<usize>)> = vec![];
    let mut res = vec![-1; n];
    for i in 0..n {
        let x = p[i];
        let j = v.binary_search_by(|(l, _)| l.cmp(&x)).unwrap_or_else(|l| l);
        if j == v.len() {
            v.push((x, vec![x]));
        } else {
            v[j] = (x, v[j].1.clone());
            v[j].1.push(x);
        }
        if v[j].1.len() == k {
            for &y in &v[j].1 {
                res[y] = i as isize + 1;
            }
            v.remove(j);
        }
    }
    for x in res {
        println!("{}", x);
    }
}
