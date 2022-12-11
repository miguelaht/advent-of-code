fn solve_first(input: &str) -> u32 {
    let pairs = input.lines().map(|l| l.split_once(","));

    pairs
        .map(|pair| {
            let (left, right) = pair.unwrap();

            let (left_bottom, left_top) = left.split_once("-").unwrap();
            let (left_bottom, left_top) = (
                left_bottom.parse::<u32>().unwrap(),
                left_top.parse::<u32>().unwrap(),
            );
            let (right_bottom, right_top) = right.split_once("-").unwrap();
            let (right_bottom, right_top) = (
                right_bottom.parse::<u32>().unwrap(),
                right_top.parse::<u32>().unwrap(),
            );

            if left_bottom.le(&right_bottom) && left_top.ge(&right_top) {
                return 1;
            }
            if right_bottom.le(&left_bottom) && right_top.ge(&left_top) {
                return 1;
            }
            0
        })
        .sum::<u32>()
}

fn solve_second(input: &str) -> u32 {
    let pairs = input.lines().map(|l| l.split_once(","));

    pairs
        .map(|pair| {
            let (left, right) = pair.unwrap();

            let (left_bottom, left_top) = left.split_once("-").unwrap();
            let (left_bottom, left_top) = (
                left_bottom.parse::<u32>().unwrap(),
                left_top.parse::<u32>().unwrap(),
            );
            let (right_bottom, right_top) = right.split_once("-").unwrap();
            let (right_bottom, right_top) = (
                right_bottom.parse::<u32>().unwrap(),
                right_top.parse::<u32>().unwrap(),
            );

            for i in left_bottom..=left_top {
                if i >= right_bottom && i <= right_top {
                    return 1;
                }
            }

            for i in right_bottom..=right_top {
                if i >= left_bottom && i <= left_top {
                    return 1;
                }
            }

            0
        })
        .sum::<u32>()
}

fn main() {
    let input = include_str!("../input.txt");
    let s = solve_first(input);

    println!("Problem 1 => {s}");

    let s = solve_second(input);

    println!("Problem 2 => {s}");
}

#[test]
fn test_first() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    assert_eq!(2, solve_first(input));
}

#[test]
fn test_second() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    assert_eq!(4, solve_second(input));
}
