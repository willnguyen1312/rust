fn check_lower_case_letters(s: &str) -> bool {
    for c in s.chars() {
        if c.is_uppercase() || c.is_numeric() || c.is_uppercase() {
            return false;
        }
    }
    true
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || !check_lower_case_letters(key) || !check_lower_case_letters(s) {
        return None;
    }

    let mut res = String::new();
    let mut iter_key = key.chars().cycle();

    for s_char in s.chars() {
        let key_char = iter_key.next().unwrap();
        res.push((((s_char as u8 - 97) + (key_char as u8 - 97) + 26) % 26 + 97) as char);
    }

    Some(res)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || !check_lower_case_letters(key) || !check_lower_case_letters(s) {
        return None;
    }

    let mut res = String::new();
    let mut iter_key = key.chars().cycle();

    for s_char in s.chars() {
        let key_char = iter_key.next().unwrap();
        res.push(((((s_char as i8 - 97) - (key_char as i8 - 97) + 26) as u8) % 26 + 97) as char);
    }

    Some(res)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = (0..200)
        .map(|_| (rand::random::<u8>() % 26 + 97) as char)
        .collect::<String>();

    let encrypted = encode(&key[..], s).unwrap();

    (key.to_string(), encrypted)
}
