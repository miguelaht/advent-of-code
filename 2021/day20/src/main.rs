use std::collections::HashSet;

fn parse_data(input: &str) -> (Vec<char>, HashSet<(i64, i64)>) {
    let mut image = HashSet::new();

    let mut input = input.split("\n\n");
    let algo: Vec<char> = input.next().unwrap().chars().collect();

    for (y, line) in input.next().unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                image.insert((y as i64, x as i64));
            }
        }
    }

    (algo, image)
}

fn _print(image: &HashSet<(i64, i64)>) -> String {
    let y_min = image.iter().map(|(y, _)| *y).min().unwrap();
    let y_max = image.iter().map(|(y, _)| *y).max().unwrap();
    let x_min = image.iter().map(|(_, x)| *x).min().unwrap();
    let x_max = image.iter().map(|(_, x)| *x).max().unwrap();

    let mut s = String::new();
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if image.contains(&(x as i64, y as i64)) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }

    s
}

fn solve(input: &str, times: usize) -> usize {
    let (algo, mut image) = parse_data(input);
    let mut swap = false;

    for _ in 0..times {
        let mut new_image = HashSet::new();
        let y_min = image.iter().map(|(y, _)| *y).min().unwrap();
        let y_max = image.iter().map(|(y, _)| *y).max().unwrap();
        let x_min = image.iter().map(|(_, x)| *x).min().unwrap();
        let x_max = image.iter().map(|(_, x)| *x).max().unwrap();

        for y in (y_min - 1)..=(y_max + 1) {
            for x in (x_min - 1)..=(x_max + 1) {
                let mut position = String::new();
                for j in (y - 1)..=(y + 1) {
                    for i in (x - 1)..=(x + 1) {
                        if i < x_min || j < y_min || i > x_max || j > y_max {
                            if swap {
                                position.push('1');
                            } else {
                                position.push('0');
                            }
                        } else if image.contains(&(j, i)) {
                            position.push('1');
                        } else {
                            position.push('0');
                        }
                    }
                }
                let position = usize::from_str_radix(&position, 2).unwrap();
                match algo[position] {
                    '#' => {
                        new_image.insert((y, x));
                    }
                    '.' => {}
                    _ => panic!("CORRUPT"),
                }
            }
        }

        image = new_image;
        if algo[0] == '#' {
            swap = !swap;
        }
    }

    image.len()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Solution 1 => {}", solve(input, 2));
    println!("Solution 2 => {}", solve(input, 50));
}

#[test]
fn test_1() {
    let input = include_str!("../test.txt");
    assert_eq!(solve(input, 2), 35);
}
#[test]
fn test_2() {
    let input = include_str!("../test.txt");
    assert_eq!(solve(input, 50), 3351);
}
