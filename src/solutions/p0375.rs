use std::cmp;

impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut ve = vec![vec![0; n + 1]; n + 1];

        for i in (1..=n).rev() {
            for j in i..=n {
                // the money need from i..j
                ve[i][j] = (i..j)
                    .map(|x| cmp::max(ve[i][x - 1], ve[x + 1][j]) + x as i32)
                    .min()
                    .unwrap_or(0);
            }
        }

        ve[1][n]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn guess_number_2() {
        assert_eq!(Solution::get_money_amount(10), 16);
    }
}
