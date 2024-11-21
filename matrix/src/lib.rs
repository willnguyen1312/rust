pub struct Matrix {
    data: Vec<u32>,
    rows: usize,
    columns: usize,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let v: Vec<&str> = input.split('\n').collect();
        let rows = v.len();
        let columns = v.get(0).unwrap_or(&"").split(' ').count();

        let data = v.join(" ").split(" ").map(|c| c.parse().unwrap()).collect();

        Matrix {
            data,
            rows,
            columns,
        }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if row_no == 0 || row_no > self.rows {
            return None;
        }

        Some(
            self.data
                .iter()
                .skip((row_no - 1) * self.columns)
                .take(self.columns)
                .cloned()
                .collect(),
        )
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if col_no > self.columns {
            return None;
        }
        Some(
            self.data
                .iter()
                .enumerate()
                .filter(|(i, _)| i % self.columns == col_no - 1)
                .map(|(_, &d)| d)
                .collect(),
        )
    }
}
