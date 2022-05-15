fn main() {
    let mut res = vec![];
    for i in 1..100 {
        res.push(i);
        res.push(i*100);
        res.push(i*10000);
    }
    println!("{}", res.len());
    for &res in &res {
        print!("{} ", res);
    }
    println!();
}
