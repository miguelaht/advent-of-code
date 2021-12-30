use std::fs::File;
use std::io::{self, BufRead};

fn walk(down: usize, right: usize, forest: &Vec<Vec<String>>) -> u32 {
    let mut trees = 0u32;
    let mut col = 0usize;
    let mut row = 0usize;
    let len = forest.first().unwrap().len();

    while row < forest.len() {
        if forest[row][col % len] == "#" {
            trees += 1;
        }
        row += down;
        col += right;
    }

    trees
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(input).lines();
    let forest: Vec<Vec<String>> = lines
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    let r1d1 = walk(1, 1, &forest);
    let r3d1 = walk(1, 3, &forest);
    let r5d1 = walk(1, 5, &forest);
    let r7d1 = walk(1, 7, &forest);
    let r1d2 = walk(2, 1, &forest);
    let s2: u32 = [r1d1, r3d1, r5d1, r7d1, r1d2].iter().product();

    println!("Solution 1 = {} ", r3d1);
    println!("Solution 2 = {} ", s2);
}
