use std::iter::from_fn;

pub fn encode(source: &str) -> String {
    let mut s = String::new();
    let mut iter = source.chars().peekable();
    while let Some(c) = iter.next() {
        match from_fn(|| iter.next_if_eq(&c)).count() {
            0 => s.push(c),
            n => s.push_str(&format!("{}{}", n + 1, c)),
        }
    }
    s
}

pub fn decode(source: &str) -> String {
    source
        .split_inclusive(|c: char| !c.is_ascii_digit())
        .map(|s| s.split_at(s.len() - 1))
        .map(|(repeat, c)| c.repeat(repeat.parse().unwrap_or(1)))
        .collect()
}
