impl Solution {
    pub fn min_swaps_couples(mut row: Vec<i32>) -> i32 {
        let mut map = row.iter().enumerate().fold(vec![0; row.len()], |mut v, (i, &x)| {
            v[x as usize] = i;
            v
        });
        let mut res = 0;
        for i in (0..row.len()).step_by(2) {
            let couple = if row[i] % 2 == 0 {
                row[i] + 1
            } else {
                row[i] - 1
            };
            if couple != row[i + 1] {
                let idx = map[couple as usize];
                map[row[i + 1] as usize] = idx;
                row.swap(i + 1, idx);
                res += 1;
            }
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn couples_holding_hands() {
        assert_eq!(1, Solution::min_swaps_couples(vec![0, 2, 1, 3]));
        assert_eq!(0, Solution::min_swaps_couples(vec![3, 2, 0, 1]));
    }
}
