pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut verses = Vec::new();

    let number_to_word = |n: u32| match n {
        0 => "no".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        _ => n.to_string(),
    };

    for i in 0..take_down {
        let current_bottles = start_bottles - i;
        let next_bottles = start_bottles - i - 1;

        let verse = format!(
            "{0} green bottle{1} hanging on the wall,\n\
            {0} green bottle{1} hanging on the wall,\n\
            And if one green bottle should accidentally fall,\n\
            There'll be {2} green bottle{3} hanging on the wall.\n",
            number_to_word(current_bottles)
                .chars()
                .next()
                .unwrap()
                .to_uppercase()
                .collect::<String>()
                + &number_to_word(current_bottles)[1..],
            if current_bottles == 1 { "" } else { "s" },
            number_to_word(next_bottles),
            if next_bottles == 1 { "" } else { "s" }
        );

        verses.push(verse);
    }

    return verses.join("\n");
}
