use crypto::digest::Digest;
use crypto::md5::Md5;

fn solution_1(input: &str) -> (u64, u64) {
    let mut hasher = Md5::new();
    let key = input.as_bytes();
    let mut five = 0;
    let mut six = 0;

    for i in 0..u64::max_value() {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut result = [0u8; 16];
        hasher.result(&mut result);

        if result[..2] == [0, 0] && result[2] <= 0x0F {
            if five == 0 {
                five = i;
            }
            if result[2] == 0 {
                six = i;
                break;
            }
        }
        hasher.reset();
    }
    (five, six)
}

fn main() {
    let input = "iwrupvqb";
    let (a, b) = solution_1(input);
    println!("Solution 1 -> {}", a);
    println!("Solution 2 -> {}", b);
}

#[test]
fn test() {
    let (a, _) = solution_1("abcdef");
    assert_eq!(609043, a);
}
