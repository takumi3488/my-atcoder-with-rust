use proconio::input;

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
    }
    let diff = ((x1-x2).abs(),(y1-y2).abs());
    let targets: Vec<(i64,i64)> = vec![
        (0,0),
        (0,2),
        (0,4),
        (1,1),
        (1,3),
        (2,4),
        (3,1),
        (3,3),
        (4,0),
        (4,2)
    ];
    if targets.iter().any(|&t|t==diff) {
        println!("Yes")
    } else {
        println!("No")
    }
}
