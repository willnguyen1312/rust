pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    (0..input.len())
        .flat_map(|row| (0..input[row].len()).map(move |column| (row, column)))
        .filter(|&(row, column)| {
            input[row][column] == input[row].iter().max().cloned().unwrap_or(0)
        })
        .filter(|&(row, column)| {
            input[row][column] == input.iter().map(|r| r[column]).min().unwrap_or(0)
        })
        .collect()
}
