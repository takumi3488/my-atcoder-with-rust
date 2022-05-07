use proconio::{input,fastout};

#[fastout]
fn main() {
    input! {
        _: isize,
        a: isize,
        b: isize,
        p: isize,
        q: isize,
        r: isize,
        s: isize
    }
    for i in p..=q {
        for j in r..=s {
            let cur = if (i - a).abs() == (j - b).abs() { '#' } else { '.' };
            print!("{}", cur);
        }
        println!();
    }
}
