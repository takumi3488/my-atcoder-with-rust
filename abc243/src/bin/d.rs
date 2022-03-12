use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        _: usize,
        x: usize,
        s: String
    }
    let b = format!("{:b}", x);
    let mut res: VecDeque<char> = b.chars().into_iter().collect();
    for si in s.chars() {
        if si.to_string() == String::from("U") {
            res.pop_back();
        } else if si.to_string() == String::from("L") {
            res.push_back("0".chars().next().unwrap());
        } else {
            res.push_back("1".chars().next().unwrap());
        }
    }
    let mut r: usize = 0;
    for (i, c) in res.iter().rev().enumerate() {
        if c.to_string() == String::from("1") {
            r += 2_usize.pow(i as u32);
        }
    }
    println!("{}", r);
}
