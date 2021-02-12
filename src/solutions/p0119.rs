impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut acc: u64 = 1;
        let mut res = vec![];
        for i in 0..=row_index {
            res.push(acc as i32);
            acc = acc * (row_index - i) as u64 / (i + 1) as u64;
        }
        res
    }
}

struct Solution;
