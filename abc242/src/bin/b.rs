use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String
    }
    for x in s.chars().iter().sorted() {
        print!("{}",x);
    }
    println!("")
}
