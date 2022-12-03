fn main() {
    let input = include_str!("../input.txt");

    let mut calories = input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|calory| calory.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    calories.sort();

    match calories.last() {
        Some(c) => println!("Problem 1 {c}"),
        None => println!("Error"),
    }

    let c: u32 = calories.iter().rev().take(3).sum();
    println!("Problem 2 {c}")
}
