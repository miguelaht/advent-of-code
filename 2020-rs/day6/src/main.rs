fn main() {
    println!(
        "Solution 1 -> {}",
        include_str!("../input.txt")
            .split("\n\n")
            .map(|a| {
                a.split_ascii_whitespace()
                    .map(|x| x.chars().collect::<Vec<char>>())
                    .flatten()
                    .collect::<Vec<char>>()
            })
            .map(|mut x| -> usize {
                x.sort();
                x.dedup();
                x.len()
            })
            .reduce(|a, b| a + b)
            .unwrap()
    );

    println!(
        "Solution 2 -> {}",
        include_str!("../input.txt")
            .split("\n\n")
            .map(|a| {
                a.split_ascii_whitespace()
                    .map(|x| x.chars().collect::<Vec<char>>())
                    .collect::<Vec<Vec<char>>>()
            })
            .map(|a| {
                let m = a.len();
                let mut count = 0;
                let first = a.get(0).unwrap();
                for c in first {
                    let ans = a.iter().filter(|x| x.contains(c)).count();
                    count += if ans == m { 1 } else { 0 };
                }
                count
            })
            .reduce(|a, b| a + b)
            .unwrap()
    );
}

#[test]
fn answers() {
    let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb"
    .split("\n\n")
    .map(|a| {
        a.split_ascii_whitespace()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
    })
    .map(|a| {
        let m = a.len();
        let mut count = 0;
        let first = a.get(0).unwrap();
        for c in first {
            let ans = a.iter().filter(|x| x.contains(c)).count();
            count += if ans == m { 1 } else { 0 };
        }
        dbg!(count);
        count
    })
    .reduce(|a, b| a + b)
    .unwrap();
    assert_eq!(input, 6);
}
