use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        mut s: Chars
    }
    s.sort_unstable();
    let res: String = s.iter().collect();
    println!("{}", res);
}
