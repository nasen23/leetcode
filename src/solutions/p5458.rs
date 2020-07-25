impl Solution {
    pub fn num_splits(s: String) -> i32 {
        let n = s.len();
        let left = s
            .bytes()
            .enumerate()
            .fold((vec![false; 26], vec![0; n]), |(mut v, mut c), (i, b)| {
                let j = (b - b'a') as usize;
                c[i] = i.checked_sub(1).map(|i| c[i]).unwrap_or(0);
                if !v[j] {
                    v[j] = true;
                    c[i] += 1;
                }
                (v, c)
            })
            .1;
        let right = s
            .bytes()
            .enumerate()
            .rev()
            .fold((vec![false; 26], vec![0; n]), |(mut v, mut c), (i, b)| {
                let j = (b - b'a') as usize;
                c[i] = *c.get(i + 1).unwrap_or(&0);
                if !v[j] {
                    v[j] = true;
                    c[i] += 1;
                }
                (v, c)
            })
            .1;
        (0..s.len() - 1)
            .filter(|&i| left[i] == right[i + 1])
            .count() as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_splits() {
        assert_eq!(Solution::num_splits("aacaba".into()), 2);
        assert_eq!(Solution::num_splits("acbadbaada".into()), 2);
    }
}
