fn main() {
    let mut counter = 0;
    let mut position = 0;
    let mut basement = false;

    include_str!("../input.txt").chars().for_each(|c| {
        counter = if c == '(' { counter + 1 } else { counter - 1 };
        if !basement {
            position += 1;
        }
        basement = if !basement { counter < 0 } else { basement };
    });

    println!("Solution 1 -> {}", counter);
    println!("Solution 2 -> {}", position);
}
