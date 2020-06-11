impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut stk = vec![];
        let mut res = vec![0; t.len()];
        'outer: for (i, x) in t.into_iter().enumerate() {
            while let Some(&(j, y)) = stk.last() {
                if y >= x {
                    stk.push((i, x));
                    continue 'outer;
                } else {
                    stk.pop();
                    res[j] = (i - j) as i32;
                }
            }
            stk.push((i, x));
        }

        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn daily_temperatures() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }
}
