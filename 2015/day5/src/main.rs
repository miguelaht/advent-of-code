use fancy_regex::Regex;

fn solution_1(input: &str) -> usize {
    input
        .lines()
        .filter(|&s| {
            let first_rule = Regex::new(r"ab|cd|pq|xy").unwrap().is_match(s).unwrap();

            return if first_rule {
                false
            } else {
                let second_rule = Regex::new(r"([a-z])\1").unwrap().is_match(s).unwrap();
                let third_rule: usize = Regex::new(r"[aeiou]")
                    .unwrap()
                    .find_iter(s)
                    .map(|x| x.unwrap().as_str())
                    .collect::<Vec<_>>()
                    .len();

                second_rule && third_rule >= 3
            };
        })
        .count()
}

fn solution_2(input: &str) -> usize {
    input
        .lines()
        .filter(|&s| {
            let first_rule = Regex::new(r"(..).*\1").unwrap().is_match(s).unwrap();
            let second_rule = Regex::new(r"(.).\1").unwrap().is_match(s).unwrap();
            first_rule && second_rule
        })
        .count()
}

fn main() {
    println!("Solution 1 -> {}", solution_1(include_str!("../input.txt")));
    println!("Solution 2 -> {}", solution_2(include_str!("../input.txt")));
}

#[test]
fn test_naughty1() {
    assert_eq!(0, solution_1("jchzalrnumimnmhp"));
    assert_eq!(0, solution_1("haegwjzuvuyypxyu"));
    assert_eq!(0, solution_1("dvszwmarrgswjxmb"));
}
#[test]
fn test_naughty_2() {
    assert_eq!(0, solution_2("uurcxstgmygtbstg"));
    assert_eq!(0, solution_2("ieodomkazucvgmuy"));
}
#[test]
fn test_nice_1() {
    assert_eq!(1, solution_1("ugknbfddgicrmopn"));
}
#[test]
fn test_nice_2() {
    assert_eq!(1, solution_2("qjhvhtzxzqqjkmpb"));
    assert_eq!(1, solution_2("xxyxx"));
}
