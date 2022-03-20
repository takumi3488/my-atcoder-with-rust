use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut res = String::new();
    let mut start = 0;
    for end in (n - k)..n {
        let u = &s[start..=end];
        let (i, &c) = u.iter().enumerate().min_by_key(|(_, &x)| x).unwrap();
        start += i + 1;
        res.push(c);
    }
    println!("{}", res);
}
