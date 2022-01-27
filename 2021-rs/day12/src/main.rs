use std::collections::{HashMap, HashSet};

fn is_small_cave(cave: &str) -> bool {
    cave.chars().next().unwrap().is_lowercase()
}

fn solve(input: &str, twice: bool) -> usize {
    let mut caves: HashMap<&str, Vec<&str>> = HashMap::new();
    input.lines().for_each(|l| {
        let (from, to) = l.split_once("-").unwrap();
        caves.entry(from).or_default().push(to);
        caves.entry(to).or_default().push(from);
    });

    let mut visited = HashSet::new();
    visited.insert("start");

    find_paths(&caves, Vec::new(), "start", visited, twice)
}

fn find_paths<'a>(
    caves: &'a HashMap<&'a str, Vec<&'a str>>,
    mut path: Vec<&'a str>,
    from: &'a str,
    visited: HashSet<&'a str>,
    twice: bool,
) -> usize {
    path.push(from);
    if from == "end" {
        return 1;
    }

    let mut count = 0;
    for to in caves.get(from).unwrap() {
        let mut visited = visited.clone();
        let mut twice = twice;
        if is_small_cave(to) {
            if visited.contains(to) {
                if twice && *to != "start" && *to != "end" {
                    twice = false;
                } else {
                    continue;
                }
            } else {
                visited.insert(*to);
            }
        }
        count += find_paths(caves, path.clone(), to, visited, twice);
    }

    count
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Solution 1 -> {}", solve(input, false));
    println!("Solution 1 -> {}", solve(input, true));
}

#[test]
fn test_1() {
    let inp = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    assert_eq!(10, solve(inp, false));
    assert_eq!(36, solve(inp, true));
}

#[test]
fn test_2() {
    let inp = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    assert_eq!(19, solve(inp, false));
    assert_eq!(103, solve(inp, true));
}
