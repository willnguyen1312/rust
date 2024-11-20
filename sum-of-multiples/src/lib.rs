pub fn sum_of_multiples(limit: u32, divs: &[u32]) -> u32 {
    (1..limit)
        .filter(|x| divs.iter().any(|d| *d != 0 && x % d == 0))
        .sum()
}
