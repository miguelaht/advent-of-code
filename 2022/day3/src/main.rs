use std::collections::HashSet;

fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u32).wrapping_sub(96)
    } else {
        (c as u32).wrapping_sub(38)
    }
}

fn get_unique_items(compartment: &str) -> HashSet<u32> {
    compartment
        .chars()
        .map(get_priority)
        .collect::<HashSet<u32>>()
}

fn solve_first(input: &str) -> u32 {
    let rucksacks = input.lines();

    rucksacks
        .map(|bag| {
            let half_str_len = bag.len() / 2;
            let (c1, c2) = bag.split_at(half_str_len);

            let first_compartment = get_unique_items(c1);

            let second_compartment = get_unique_items(c2);

            first_compartment
                .intersection(&second_compartment)
                .sum::<u32>()
        })
        .sum()
}

fn solve_second(input: &str) -> u32 {
    let rucksacks = input.lines().collect::<Vec<&str>>();

    rucksacks
        .chunks(3)
        .map(|bag| {
            let group = bag
                .iter()
                .map(|c| get_unique_items(*c))
                .collect::<Vec<HashSet<u32>>>();

            group[0]
                .intersection(&group[1])
                .map(|i| *i)
                .collect::<HashSet<u32>>()
                .intersection(&group[2])
                .sum::<u32>()
        })
        .sum()
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
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    assert_eq!(157, solve_first(input));
}

#[test]
fn test_second() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    assert_eq!(70, solve_second(input));
}
