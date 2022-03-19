use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    if (a.last().unwrap() + 1 - n) % 2 == 0 {
        println!("Bob")
    } else {
        println!("Alice")
    }
}
