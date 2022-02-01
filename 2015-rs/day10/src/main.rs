fn solution_1(inp: String, rounds: u16) -> String {
    let mut ans = inp;

    for _ in 0..rounds {
        let mut new_ans = String::new();

        let mut chars = ans.chars().into_iter().collect::<Vec<char>>();
        chars.push(' ');
        let mut curr = chars[0];
        let mut count: u8 = 0;

        for c in chars {
            let temp = c;
            if temp == curr {
                count += 1;
            } else {
                new_ans = format!("{}{}{}", new_ans, count, curr);
                curr = c;
                count = 1;
            }
        }
        ans = new_ans;
    }

    ans
}

fn iterate(s: String) -> String {
    let mut it = s.chars().peekable();
    let mut res = vec![];
    loop {
        if let Some(c) = it.next() {
            let mut count = 1;
            while it.peek() == Some(&c) {
                it.next();
                count += 1;
            }
            res.push(format!("{}{}", count, c));
        } else {
            break;
        }
    }

    res.join("")
}

fn main() {
    /* let input = String::from("3113322113");
    let first = solution_1(input, 40);
    println!("Solution 1 -> {}", first.len());
    let second = solution_1(first, 10);
    println!("Solution 2 -> {}", second.len()); */

    let mut s = "3113322113".to_string();
    for _ in 0..40 {
        s = iterate(s);
    }
    println!("Part 1: {}", s.len());
    for _ in 0..10 {
        s = iterate(s);
    }
    println!("Part 2: {}", s.len());
}

#[test]
fn test_solution_1() {
    assert_eq!("11", solution_1("1".to_string(), 1));
    assert_eq!("21", solution_1("11".to_string(), 1));
    assert_eq!("1211", solution_1("21".to_string(), 1));
    assert_eq!("111221", solution_1("1211".to_string(), 1));
    assert_eq!("312211", solution_1("111221".to_string(), 1));
}
