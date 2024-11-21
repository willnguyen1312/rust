pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                rotate_char(c, key)
            } else {
                c
            }
        })
        .collect()
}

fn rotate_char(ch: char, offset: i8) -> char {
    let base = if ch.is_ascii_lowercase() { 97 } else { 65 };
    let idx = (ch as u8 - base) as i8;
    let new_idx = rotate_index(idx, offset);
    (base + new_idx) as char
}

fn rotate_index(idx: i8, offset: i8) -> u8 {
    let mut v = idx + offset;
    while v < 0 {
        v += 26;
    }
    while v >= 26 {
        v -= 26;
    }
    v as u8
}
