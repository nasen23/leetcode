impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        let mut nums = (1..=n)
            .map(|x| (x as u8 + b'0') as char)
            .collect::<Vec<_>>();
        let fact = (1..=n).fold(vec![1; n as usize + 1], |mut v, x| {
            v[x as usize] = v[x as usize - 1] * x;
            v
        });
        let mut res = String::new();
        k -= 1;
        for i in (0..n as usize).rev() {
            let j = (k / fact[i]) as usize;
            res.push(nums.remove(j));
            k %= fact[i];
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation_sequence() {
        assert_eq!(Solution::get_permutation(3, 3), "213");
        assert_eq!(Solution::get_permutation(4, 9), "2314");
    }
}
