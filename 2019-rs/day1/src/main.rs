fn main() {
    println!(
        "Solution 1 -> {}",
        include_str!("../input.txt")
            .lines()
            .map(|n| n.parse::<i64>().unwrap())
            .map(|f| f / 3 - 2)
            .sum::<i64>()
    );

    println!(
        "Solution 2 -> {}",
        include_str!("../input.txt")
            .lines()
            .map(|n| n.parse::<i64>().unwrap())
            .map(|f| {
                let mut total = 0;
                let mut t = f / 3 - 2;
                loop {
                    if t > 0 {
                        total += t;
                        t = t / 3 - 2;
                    } else {
                        break;
                    }
                }

                total
            })
            .sum::<i64>()
    );
}

#[test]
fn test_solution_2() {
    assert_eq!(
        50346,
        "100756"
            .lines()
            .map(|n| n.parse::<i64>().unwrap())
            .map(|f| {
                let mut total = 0;
                let mut t = f / 3 - 2;
                loop {
                    if t > 0 {
                        total += t;
                        t = t / 3 - 2;
                    } else {
                        break;
                    }
                }

                total
            })
            .sum::<i64>()
    );
}
