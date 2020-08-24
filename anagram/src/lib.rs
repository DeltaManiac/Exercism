use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    fn sort(word: &str) -> Vec<char> {
        let mut letters: Vec<char> = word.chars().collect();
        letters.sort();
        letters
    }

    let word = word.to_lowercase();
    let sorted_word = sort(&word);
    possible_anagrams
        .iter()
        .cloned()
        .filter(|&anagram| {
            let item = anagram.to_lowercase();
            sort(&item) == sorted_word && item != word
        })
        .collect::<HashSet<&'a str>>()
}
