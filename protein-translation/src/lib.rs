pub fn translate(rna: &str) -> Option<Vec<&str>> {
    rna.as_bytes()
        .chunks(3)
        .map(decode)
        .take_while(|&x| x != Some("STOP"))
        .collect()
}

fn decode(input: &[u8]) -> Option<&'static str> {
    match input {
        b"AUG" => Some("Methionine"),
        b"UUU" | b"UUC" => Some("Phenylalanine"),
        b"UUA" | b"UUG" => Some("Leucine"),
        b"UCU" | b"UCC" | b"UCA" | b"UCG" => Some("Serine"),
        b"UAU" | b"UAC" => Some("Tyrosine"),
        b"UGU" | b"UGC" => Some("Cysteine"),
        b"UGG" => Some("Tryptophan"),
        b"UAA" | b"UAG" | b"UGA" => Some("STOP"),
        _ => None,
    }
}
