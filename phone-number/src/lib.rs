pub fn number(user_number: &str) -> Option<String> {
    let number: String = user_number.chars().filter(|c| c.is_numeric()).collect();

    if number.len() == 10 && check_format(&number) {
        Some(number)
    } else if number.len() == 11 && &number[0..1] == "1" && check_format(&number[1..]) {
        Some(String::from(&number[1..]))
    } else {
        None
    }
}

fn check_format(number: &str) -> bool {
    number
        .char_indices()
        .all(|(index, char)| (index == 0 || index == 3) && char >= '2' || index != 0 && index != 3)
}
