use fancy_regex::Regex;

fn get_new_password(inp: String, iterations: u8) -> String {
    let mut password = inp;
    let mut iterations = iterations;

    return loop {
        // may not contain i | o | l
        let banned = Regex::new("i|o|l").unwrap().is_match(&password).unwrap();

        // may contain 2 pairs
        let pairs = Regex::new(r"(.)\1")
            .unwrap()
            .find_iter(&password)
            .into_iter()
            .map(|m| m.unwrap().as_str())
            .collect::<Vec<_>>()
            .len();

        let mut start = 0;
        let valid = loop {
            if start + 3 >= password.len() {
                break false;
            }
            let sub = &password[start..start + 3]
                .chars()
                .into_iter()
                .map(|c| c as u32)
                .collect::<Vec<u32>>();

            if sub[0] + 2 == sub[1] + 1 && sub[1] + 1 == sub[2] {
                break true;
            }
            start += 1;
        };

        if !banned && pairs >= 2 && valid {
            iterations -= 1;
            if iterations == 0 {
                break password;
            }
        }

        // increment
        let mut index = password.len() - 1;
        let mut pass = password.chars().into_iter().collect::<Vec<char>>();

        loop {
            let curr = pass.get_mut(index).unwrap();
            *curr = char::from_u32(*curr as u32 + 1).unwrap();
            if *curr <= 'z' || index == 0 {
                password = pass.iter().collect::<String>();
                break;
            } else {
                *curr = 'a';
                index -= 1;
            }
        }
    };
}

fn main() {
    let old_password = "hepxcrrq";

    let first = get_new_password(old_password.to_string(), 1u8);
    println!("solution 1 -> {}", first);

    let second = get_new_password(first, 1u8);
    println!("solution 2 -> {}", second);
}
