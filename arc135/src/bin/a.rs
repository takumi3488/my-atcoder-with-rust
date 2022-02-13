use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        x: usize
    }
    let v = vec![x];
    let mut m: HashMap<usize, usize> = HashMap::new();
    m.insert(1, 1);
    m.insert(2, 2);
    m.insert(3, 3);
    m.insert(4, 4);
    println!("{}", calc(v, &mut m));
}

fn calc(x: Vec<usize>, m: &mut HashMap<usize, usize>) -> usize {
    let mut res = 1usize;
    for i in x {
        if m.get(&i).is_some() {
            res = (res * m.get(&i).unwrap()) % 998244353;
        } else {
            let a = calc(vec![i / 2, (i + 1) / 2], m);
            m.insert(i, a);
            res = (res * a) % 998244353;
        }
    }
    res
}
