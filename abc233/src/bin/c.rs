fn main() {
    proconio::input! {
        n: u128,
        x: u128,
        a: [[u128]; n]
    };
    let mut dp = vec![1_u128];

    for ai in a.iter() {
        dp = dp.iter().flat_map(|&i| ai.iter().map(move |&j| i * j)).collect();
    }

    println!("{:}", dp.iter().filter(|&i| *i == x).count());
}
