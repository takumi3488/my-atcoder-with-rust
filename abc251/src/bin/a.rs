use proconio::{input,fastout, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars
    }
    let c = if s.len() == 1 {6} else if s.len() == 2 {3} else {2};
    for _ in 0..c {
        print!("{}", s.iter().collect::<String>());
    }
    println!();
}
