use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    }
    if b - a == 1 || b - a == 9 {
        println!("Yes");
    } else {
        println!("No");
    }
}
