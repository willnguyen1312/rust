/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut encoding_sentence = String::new();
    let mut count: u32 = 0;

    for letter in plain.to_lowercase().chars() {
        if !(letter.is_ascii_punctuation() || letter.is_whitespace()) {
            if count == 5 {
                encoding_sentence.push(' ');
                count = 0;
            }
            count += 1;
            encoding_sentence.push(get_match(letter))
        }
    }

    return encoding_sentence;
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut dencoding_sentence = String::new();
    for letter in cipher.replace(" ", "").chars() {
        dencoding_sentence.push(get_match(letter));
    }

    return dencoding_sentence;
}

/// Get the character that match with passed character
fn get_match(letter: char) -> char {
    match letter {
        'a' => 'z',
        'b' => 'y',
        'c' => 'x',
        'd' => 'w',
        'e' => 'v',
        'f' => 'u',
        'g' => 't',
        'h' => 's',
        'i' => 'r',
        'j' => 'q',
        'k' => 'p',
        'l' => 'o',
        'm' => 'n',
        'n' => 'm',
        'o' => 'l',
        'p' => 'k',
        'q' => 'j',
        'r' => 'i',
        's' => 'h',
        't' => 'g',
        'u' => 'f',
        'v' => 'e',
        'w' => 'd',
        'x' => 'c',
        'y' => 'b',
        'z' => 'a',
        _ => letter,
    }
}
