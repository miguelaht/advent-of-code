use itertools::Itertools;

fn main() {
    let solution_1 = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .combinations(2)
        .filter(|x| x.iter().sum::<usize>() == 2020)
        .next()
        .map(|x| x.iter().product::<usize>())
        .unwrap();

    println!("Solution 1: {}", solution_1);

    let solution_2 = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .combinations(3)
        .filter(|x| x.iter().sum::<usize>() == 2020)
        .next()
        .map(|x| x.iter().product::<usize>())
        .unwrap();

    println!("Solution 2: {}", solution_2);
}
