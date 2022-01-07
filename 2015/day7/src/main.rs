use std::collections::HashMap;

fn solution_1(input: &str) -> HashMap<&str, u16> {
    let mut ans: HashMap<&str, u16> = HashMap::new();
    input.lines().for_each(|l| {
        let command: Vec<&str> = l.split_ascii_whitespace().collect();

        match command.len() {
            3 => {
                let a = if command[0].chars().all(char::is_numeric) {
                    command[0].parse::<u16>().unwrap()
                } else {
                    *ans.get(command[0]).unwrap_or(&0)
                };
                let e = ans.entry(command[2]).or_default();
                *e = a;
            }
            4 => {
                if ans.contains_key(&command[1]) {
                    dbg!(&command);
                    let a = *ans.get(&command[1]).unwrap_or(&0);
                    let b = ans.entry(command[3]).or_default();

                    *b = !a;
                }
            }
            _ => {
                if ans.contains_key(&command[0]) {
                    let a = *ans.get(command[0]).unwrap_or(&0);
                    let b = if command[2].chars().all(char::is_numeric) {
                        command[2].parse::<u16>().unwrap()
                    } else {
                        *ans.get(command[2]).unwrap_or(&0)
                    };

                    let op = command[1];
                    let e = ans.entry(command[4]).or_default();

                    *e = match op {
                        "AND" => a & b,
                        "OR" => a | b,
                        "LSHIFT" => a << b,
                        "RSHIFT" => a >> b,
                        _ => *e,
                    };
                }
            }
        }
    });

    ans
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Solution 1 -> {:?}", solution_1(input));
}

#[test]
fn test_1() {
    let ans: HashMap<&str, u16> = [
        ("d", 72u16),
        ("e", 507u16),
        ("f", 492u16),
        ("g", 114u16),
        ("h", 65412u16),
        ("i", 65079u16),
        ("x", 123u16),
        ("y", 456u16),
    ]
    .iter()
    .cloned()
    .collect();

    let input = include_str!("../test.txt");
    assert_eq!(ans, solution_1(input));
}
