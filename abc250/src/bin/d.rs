use num::integer;
use proconio::{fastout, input};

#[allow(unused_assignments)]
#[fastout]
fn main() {
    input! {
        n: usize
    }
    let primes = list_primes(integer::cbrt(n));
    let mut res = 0;
    for (i, &p) in primes.iter().enumerate() {
        let mut tmp = 0;
        for &q in primes[i + 1..].iter() {
            if p * q * q * q <= n {
                tmp += 1;
            } else {
                break;
            }
        }
        if tmp > 0 {
            res += tmp;
            tmp = 0;
        } else {
            break;
        }
    }
    println!("{}", res);
}

fn list_primes(limit: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for p in 0..=limit {
        if !is_prime[p] {
            continue;
        }
        primes.push(p as usize);
        for i in (p * p..=limit).step_by(p) {
            is_prime[i] = false;
        }
    }
    primes
}
