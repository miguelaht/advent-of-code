use std::convert::TryInto;

fn main() {
    let solution_1: Vec<&str> = include_str!("../input.txt")
        .lines()
        .filter(|s| {
            let parts: Vec<&str> = s.split(" ").collect();
            let bounds: Vec<u32> = parts[0]
                .split("-")
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            let count: u32 = String::from(parts[2])
                .matches(parts[1].chars().next().unwrap())
                .count()
                .try_into()
                .unwrap();

            return count >= bounds[0] && count <= bounds[1];
        })
        .collect();

    println!("Solution 1 {} ", solution_1.iter().count());

    let solution_2: Vec<&str> = include_str!("../input.txt")
        .lines()
        .filter(|s| {
            let parts: Vec<&str> = s.split(" ").collect();
            let positions: Vec<usize> = parts[0]
                .split("-")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            let f =
                parts[2].chars().nth(positions[0] - 1).unwrap() == parts[1].chars().next().unwrap();
            let s = parts[2].chars().nth(positions[1] - 1).unwrap_or(' ')
                == parts[1].chars().next().unwrap();
            return if f && s { false } else { f || s };
        })
        .collect();

    println!("Solution 2 {} ", solution_2.iter().count());
}
