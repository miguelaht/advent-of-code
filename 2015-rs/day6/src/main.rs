use fancy_regex::Regex;
use std::collections::{HashMap, HashSet};

const CMD: &str = r"(turn on|turn off|toggle)";
const PARSE: &str = r"(\d+),(\d+)";

fn solve(input: &str) -> usize {
    let mut grid = HashSet::new();

    for line in input.lines() {
        let cmd = Regex::new(CMD)
            .unwrap()
            .find_iter(line)
            .map(|x| x.unwrap().as_str())
            .collect::<Vec<_>>()[0];

        let positions = Regex::new(PARSE)
            .unwrap()
            .find_iter(line)
            .map(|x| {
                x.unwrap()
                    .as_str()
                    .split_once(',')
                    .map(|n| (n.0.parse::<usize>().unwrap(), n.1.parse::<usize>().unwrap()))
                    .unwrap()
            })
            .collect::<Vec<_>>();

        let (x_from, y_from) = positions[0];
        let (x_to, y_to) = positions[1];

        for x in x_from..=x_to {
            for y in y_from..=y_to {
                match cmd {
                    "turn on" => {
                        grid.insert((x, y));
                    }
                    "turn off" => {
                        grid.remove(&(x, y));
                    }
                    "toggle" => {
                        if grid.contains(&(x, y)) {
                            grid.remove(&(x, y));
                        } else {
                            grid.insert((x, y));
                        }
                    }
                    _ => panic!(),
                };
            }
        }
    }

    grid.len()
}

fn solve_2(input: &str) -> usize {
    let mut grid = HashMap::new();

    for line in input.lines() {
        let cmd = Regex::new(CMD)
            .unwrap()
            .find_iter(line)
            .map(|x| x.unwrap().as_str())
            .collect::<Vec<_>>()[0];

        let positions = Regex::new(PARSE)
            .unwrap()
            .find_iter(line)
            .map(|x| {
                x.unwrap()
                    .as_str()
                    .split_once(',')
                    .map(|n| (n.0.parse::<usize>().unwrap(), n.1.parse::<usize>().unwrap()))
                    .unwrap()
            })
            .collect::<Vec<_>>();

        let (x_from, y_from) = positions[0];
        let (x_to, y_to) = positions[1];

        for x in x_from..=x_to {
            for y in y_from..=y_to {
                match cmd {
                    "turn on" => {
                        *grid.entry((x, y)).or_insert(0) += 1;
                    }
                    "turn off" => {
                        let e = grid.entry((x, y)).or_insert(0);
                        if *e > 0 {
                            *e -= 1;
                        }
                    }
                    "toggle" => {
                        *grid.entry((x, y)).or_insert(0) += 2;
                    }
                    _ => panic!(),
                };
            }
        }
    }

    grid.values().sum()
}
fn main() {
    let input = include_str!("../input.txt");
    println!("Solution 1 -> {}", solve(input));
    println!("Solution 2 -> {}", solve_2(input));
}
