pub fn is_armstrong_number(num: u32) -> bool {
    if num < 10 {
        return true;
    }

    let digits: Vec<char> = num.to_string().chars().collect();
    let power = digits.len() as u32;
    let sum: u32 = digits
        .iter()
        .map(|d| {
            let digit = d.to_digit(10).unwrap();

            digit.pow(power)
        })
        .sum();

    sum == num
}
