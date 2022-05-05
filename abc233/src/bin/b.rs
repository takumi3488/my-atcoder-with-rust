use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        l: Usize1,
        r: usize,
        s: Chars
    }
    print!("{}", &s[..l].iter().collect::<String>());
    print!("{}", s[l..r].iter().rev().collect::<String>());
    println!("{}", &s[r..].iter().collect::<String>());
}
