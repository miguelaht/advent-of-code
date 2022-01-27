use fancy_regex::Regex;

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let chars: isize = Regex::new("/\"/")
                .unwrap()
                .find_iter(l)
                .count()
                .try_into()
                .unwrap();
            let code: isize = l.len().try_into().unwrap();
            let code = code - chars;

            dbg!(l, code, chars);
            code - chars
        })
        .sum::<isize>()
        .try_into()
        .unwrap()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Solution 1 -> {}", solve(input));
    // let a = "aaa\"aaa";
    // println!("{} {}", a, a.len());
    // let x = Regex::new("\"").unwrap().find_iter(a).count();
    // println!("{:?}", x);
}

#[test]
fn test_solution_1() {
    let input = include_str!("../test.txt");
    assert_eq!(12, solve(input));
}
