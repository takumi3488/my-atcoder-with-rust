use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        t: Chars
    }
    let mut pos = (0, 0);
    let mut dir = (1, 0);
    for t in t {
        if t == "S".chars().last().unwrap() {
            pos.0 += dir.0;
            pos.1 += dir.1;
        } else {
            dir = (dir.1, -dir.0);
        }
    }
    println!("{} {}", pos.0, pos.1);
}
