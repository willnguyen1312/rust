pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(mut max_weight: u32, items: &[Item]) -> u32 {
    items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            if item.weight > max_weight {
                0
            } else {
                maximum_value(max_weight - item.weight, &items[index + 1..]) + item.value
            }
        })
        .max()
        .unwrap_or(0)
}
