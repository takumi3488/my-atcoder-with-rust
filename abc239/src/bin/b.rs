use proconio::input;

fn main() {
    input! {
        mut x: String
    }
    let last = x.pop().unwrap();
    let r: isize = x.parse().unwrap();
    if r < 0 && last.to_string() != "0" {
        println!("{}", r - 1);
    } else {
        println!("{}", r);
    }
}
