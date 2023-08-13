use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let x = solve(&n);

    println!("{}", x);
}

fn solve(n: &usize) -> usize {
    *n + 2
}

#[test]
fn {{project-name}}_f() {
    assert_eq!(solve(&1), 3);
}