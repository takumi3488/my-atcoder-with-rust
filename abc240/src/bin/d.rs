use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }
    let mut balls: Vec<usize> = vec![];
    for i in 0..n {
        let ai = a[i];
        balls.push(ai);
        if balls.len() >= ai {
            balls = delete_balls(&balls, ai);
        }
        println!("{}", &balls.iter().count());
    }
}

fn delete_balls(balls: &Vec<usize>, last: usize) -> Vec<usize> {
    if balls
        .get((balls.len() - last)..)
        .unwrap()
        .to_vec()
        .iter()
        .all(|&b| b == last)
    {
        return delete_balls(
            &balls.get(..(balls.len() - last)).unwrap().to_vec(),
            *balls.last().unwrap(),
        );
    }
    return balls.to_vec();
}
