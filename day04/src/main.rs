const INPUT: &str = "ckczppom";

fn starts_with_five_zeros(hash: [u8; 16]) -> bool {
    hash[0] == 0 && hash[1] == 0 && hash[2] <= 0x0f
}

fn compute_hash(key: &str, number: i32) -> [u8; 16] {
    let str = format!("{}{}", key, number);
    md5::compute(str).0
}

fn find_lowest_number(key: &str) -> i32 {
    (1..)
        .map(|x| (x, compute_hash(key, x)))
        .find(|(_, hash)| starts_with_five_zeros(*hash))
        .unwrap()
        .0
}

fn main() {
    println!("{}", find_lowest_number(INPUT));
}

#[test]
fn test_sample() {
    let key = "abcdef";
    let number = 609043;
    let hash = compute_hash(key, number);
    assert!(starts_with_five_zeros(hash));
}
