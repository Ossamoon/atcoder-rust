use num::Integer;
use proconio::input;

fn main() {
    input! (
        n: u64,
        a: [u64; n],
    );

    let x = solve(&n, &a);
    println!("{}", x);
}

fn solve(n: &u64, a: &[u64]) -> u64 {
    let sum: u64 = a.iter().sum();
    let (q, r) = sum.div_rem(&n);

    if r == 0 {
        let mut count = 0;
        for an in a {
            if *an > q {
                count += *an - q;
            }
        }
        count
    } else {
        let mut count_left = 0;
        let mut count_right = 0;
        for an in a {
            if *an < q {
                count_left += q - *an;
            }
            if *an > q + 1 {
                count_right += *an - q - 1;
            }
        }
        std::cmp::max(count_left, count_right)
    }
}

#[test]
fn abc313_c() {
    assert_eq!(solve(&4, &[4, 7, 3, 7]), 3);
    assert_eq!(solve(&1, &[313]), 0);
    assert_eq!(
        solve(
            &10,
            &[999999997, 999999999, 4, 3, 2, 4, 999999990, 8, 999999991, 999999993]
        ),
        2499999974
    );
}
