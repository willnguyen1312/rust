pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut steps = 0;
    let mut num = n;

    while num != 1 {
        if num % 2 == 0 {
            num /= 2;
        } else {
            num = 3 * num + 1;
        }
        steps += 1;
    }

    Some(steps)
}
