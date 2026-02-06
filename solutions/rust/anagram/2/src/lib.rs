use std::{collections::HashMap, collections::HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    let mut word_chars_count = HashMap::new();

    let word = word.to_lowercase();

    word.chars().for_each(|c| {
        word_chars_count.insert(c, word_chars_count.get(&c).unwrap_or(&0) + 1);
    });

    possible_anagrams.iter().for_each(|&w| {
        let w_lower = w.to_lowercase();
        let mut word_chars_hash = HashMap::new();

        w_lower.chars().for_each(|c| {
            word_chars_hash.insert(c, word_chars_hash.get(&c).unwrap_or(&0) + 1);
        });

        if word_chars_hash == word_chars_count && w_lower != word {
            anagrams.insert(w);
        }
    });

    anagrams
}
