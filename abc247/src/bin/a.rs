use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars
    }
    print!("0");
    for x in &s[0..3] {
        print!("{}", x);
    }
    println!()
}
