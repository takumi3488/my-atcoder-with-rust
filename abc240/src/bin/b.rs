use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: u32,
        a: [i64;n]
    }
    println!("{}",a.iter().unique().count())
}
