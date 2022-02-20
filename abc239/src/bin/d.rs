use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64
    }
    let primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];
    for i in a..=b {
        if !primes.iter().any(|&p| i + c <= p && p <= i + d) {
            println!("Takahashi");
            return;
        }
    }
    println!("Aoki");
}
