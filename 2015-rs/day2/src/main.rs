fn main() {
    println!(
        "Solution 1 -> {}",
        include_str!("../input.txt")
            .lines()
            .map(|c| -> usize {
                let nums = c
                    .split("x")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                let mut m = [nums[0] * nums[1], nums[1] * nums[2], nums[2] * nums[0]];
                m.sort();
                let min: usize = m[0];
                let sum: usize = m.iter().sum();
                min + (2 * sum)
            })
            .sum::<usize>()
    );

    println!(
        "Solution 2 -> {}",
        include_str!("../input.txt")
            .lines()
            .map(|c| -> usize {
                let nums = c
                    .split("x")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                let mut m = [nums[0] + nums[1], nums[1] + nums[2], nums[2] + nums[0]];
                m.sort();
                let min: usize = m[0];
                (min * 2) + nums.iter().product::<usize>()
            })
            .sum::<usize>()
    );
}

#[test]
fn test_solution_1() {
    assert_eq!(
        58,
        "2x3x4"
            .lines()
            .map(|c| -> usize {
                let nums = c
                    .split("x")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                let mut m = [nums[0] * nums[1], nums[1] * nums[2], nums[2] * nums[0]];
                m.sort();
                let min: usize = m[0];
                let sum: usize = m.iter().sum();
                println!("{}, {}", min, sum);
                min + (2 * sum)
            })
            .sum::<usize>()
    );
}

#[test]
fn test_solution_2() {
    assert_eq!(
        34,
        "2x3x4"
            .lines()
            .map(|c| -> usize {
                let nums = c
                    .split("x")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                let mut m = [nums[0] + nums[1], nums[1] + nums[2], nums[2] + nums[0]];
                m.sort();
                let min: usize = m[0];
                (min * 2) + nums.iter().product::<usize>()
            })
            .sum::<usize>()
    );
}
