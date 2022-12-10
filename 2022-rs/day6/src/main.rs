use std::collections::HashSet;

pub fn solve(input: &str, n: usize) -> u32 {
    let chrs = input.chars().collect::<Vec<char>>();

    let mut counter = 0usize;

    loop {
        let s = &chrs[counter..counter + n];

        let d: HashSet<char> = HashSet::from_iter(s.to_vec());

        if d.len() == n {
            break;
        }

        counter += 1;
    }

    (counter + n) as u32
}

fn main() {
    let input = include_str!("../input.txt");

    let p1 = solve(input, 4);

    println!("Problem 1 {p1}");
    let p2 = solve(input, 14);

    println!("Problem 2 {p2}");
}

#[test]
fn test_first() {
    use std::collections::HashMap;
    let mut input = HashMap::<&str, u32>::new();
    input.insert("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7);
    input.insert("bvwbjplbgvbhsrlpgdmjqwftvncz", 5);
    input.insert("nppdvjthqldpwncqszvftbrmjlhg", 6);
    input.insert("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10);
    input.insert("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11);

    for (k, v) in input.into_iter() {
        assert_eq!(v, solve(k, 4));
    }
}

#[test]
fn test_second() {
    use std::collections::HashMap;
    let mut input = HashMap::<&str, u32>::new();
    input.insert("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19);
    input.insert("bvwbjplbgvbhsrlpgdmjqwftvncz", 23);
    input.insert("nppdvjthqldpwncqszvftbrmjlhg", 23);
    input.insert("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29);
    input.insert("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26);

    for (k, v) in input.into_iter() {
        assert_eq!(v, solve(k, 14));
    }
}
