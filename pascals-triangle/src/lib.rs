pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            row_count: row_count,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.row_count)
            .map(|r| PascalsTriangle::calc(r))
            .collect()
    }

    fn calc(r: u32) -> Vec<u32> {
        let mut ret = vec![1];

        for p in 1..(r + 1) {
            if let Some(&last) = ret.last() {
                ret.push((last * (r + 1 - p)) / p)
            }
        }
        ret
    }
}
