pub fn count(lines: &[&str]) -> u32 {
    let v: Vec<Vec<char>> = lines
        .iter()
        .cloned()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let mut cnt = 0;

    if v.len() < 2 || v[0].len() < 2 {
        return cnt;
    }

    for i0 in 0..v.len() - 1 {
        for j0 in 0..v[0].len() - 1 {
            if v[i0][j0] == '+' {
                for j1 in j0 + 1..v[0].len() {
                    if v[i0][j1] == '+' && v[i0][j0..=j1].iter().all(|&c| c == '+' || c == '-') {
                        for i1 in i0 + 1..v.len() {
                            if v[i1][j0] == '+'
                                && v[i1][j1] == '+'
                                && (i0..=i1).map(|i| v[i][j0]).all(|c| c == '+' || c == '|')
                                && (i0..=i1).map(|i| v[i][j1]).all(|c| c == '+' || c == '|')
                                && v[i1][j0..=j1].iter().all(|&c| c == '+' || c == '-')
                            {
                                cnt += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    cnt
}
