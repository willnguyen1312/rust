const M: i32 = 26;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    match find_mmi(a, M) {
        Some(_) => Ok(encoded(plaintext, a, b)),
        None => Err(AffineCipherError::NotCoprime(a)),
    }
}

fn encoded(plaintext: &str, a: i32, b: i32) -> String {
    plaintext
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| encode_char(c, a, b))
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

fn encode_char(c: char, a: i32, b: i32) -> char {
    match c {
        '0'..='9' => c,
        _ => {
            let x = c as i32 - 'a' as i32;
            let num = (a * x + b).rem_euclid(M);
            char::from(num as u8 + 'a' as u8)
        }
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    match find_mmi(a, M) {
        Some(mmi) => Ok(decoded(ciphertext, b, mmi)),
        None => Err(AffineCipherError::NotCoprime(a)),
    }
}

fn decoded(ciphertext: &str, b: i32, mmi: i32) -> String {
    ciphertext
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| decode_char(c, b, mmi))
        .collect()
}

fn decode_char(c: char, b: i32, mmi: i32) -> char {
    match c {
        '0'..='9' => c,
        _ => {
            let y = c as i32 - 'a' as i32;
            let num = (mmi * (y - b)).rem_euclid(M);
            char::from(num as u8 + 'a' as u8)
        }
    }
}

fn find_mmi(a: i32, m: i32) -> Option<i32> {
    (1..m).into_iter().find(|n| a * n % m == 1)
}
