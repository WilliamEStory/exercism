pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => (0..(s)).fold(1, |acc, x| {
            println!("x: {}, acc: {}", x, acc);
            2_u64.pow(x)
        }),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
