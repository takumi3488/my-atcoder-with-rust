use proconio::{fastout, input};
use regex::Regex;

#[fastout]
fn main() {
    input! {
        k: usize
    }
    let s = format!("{:64b}", k).replace("1", "2");
    let re = Regex::new(r"^[^2]+2").unwrap();
    println!("{}", re.replace_all(&s, "2"));
}
