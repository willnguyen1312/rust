use std::collections::HashSet;

pub fn check(word: &str) -> bool {
    let mut seen = HashSet::new();
    word.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| seen.insert(c))
}
