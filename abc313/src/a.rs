use proconio::input;

pub fn handler() {
    input! {
        n: i32,
        p: [i32; n],
    }

    let x = solve(&p);

    println!("{}", x);
}

fn solve(p: &[i32]) -> i32 {
    let (max_value, max_index) =
        p.iter()
            .enumerate()
            .fold((0, 0), |(max_value, max_index), (i, &value)| {
                if value >= max_value {
                    (value, i)
                } else {
                    (max_value, max_index)
                }
            });

    if max_index == 0 {
        0
    } else {
        max_value - p[0] + 1
    }
}

#[test]
fn abc313_a() {
    assert_eq!(solve(&[5, 15, 2, 10]), 11);
    assert_eq!(solve(&[15, 5, 2, 10]), 0);
    assert_eq!(solve(&[100, 100, 100]), 1);
}
