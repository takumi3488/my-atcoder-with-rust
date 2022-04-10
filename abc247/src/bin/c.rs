use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize
    }
    sub(n);
    println!()
}

fn sub(n: usize) {
    if n == 1 {
        print!("1 ");
        return;
    }
    sub(n - 1);
    print!("{} ", n);
    sub(n - 1);
}
