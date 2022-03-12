use proconio::input;

fn main() {
    input! {
        mut v: i32,
        a: [i32; 3]
    }
    let mut i = 0;
    while v >= 0 {
        v -= a[i];
        i = (i + 1) % 3;
    }
    let res = vec!["F", "M", "T"];
    println!("{}", res[(i + 2) % 3]);
}
