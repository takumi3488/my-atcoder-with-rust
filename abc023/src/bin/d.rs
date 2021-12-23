use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        hs: [(usize, usize); n]
    }
    let mut a = hs.iter().map(|&hisi| hisi.0).max().unwrap();
    let mut b = hs
        .iter()
        .map(|&hisi| hisi.0 + hisi.1 * (n - 1))
        .max()
        .unwrap();
    while b - a > 1 {
        if is_ok(&hs, (a+b)/2) {
            b = (a+b)/2
        } else {
            a = (a+b)/2
        }
    }
    println!("{}", b)
}

fn is_ok(hs: &Vec<(usize, usize)>, x: usize) -> bool {
    if hs.iter().filter(|&hisi| hisi.0 > x).count() > 0 {
        return false;
    }
    hs.iter()
        .map(|&hisi| (x - hisi.0) / hisi.1)
        .sorted()
        .iter()
        .enumerate()
        .all(|(i, t)| i <= *t)
}
