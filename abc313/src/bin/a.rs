use proconio::input;

fn main() {
    input! {
        n: i32,
        p: [i32; n],
    }

    let x = solve(&p);

    println!("{}", x);
}

fn solve(p: &[i32]) -> i32 {
    if p.len() == 1 {
        return 0;
    }

    let max = *p[1..].iter().max().unwrap();

    if p[0] > max {
        0
    } else {
        max - p[0] + 1
    }
}

#[test]
fn abc313_a() {
    assert_eq!(solve(&[5, 15, 2, 10]), 11);
    assert_eq!(solve(&[15, 5, 2, 10]), 0);
    assert_eq!(solve(&[100, 100, 100]), 1);
}
