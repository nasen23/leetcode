struct Solution;

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (1..n + 1).into_iter().take_while(|x| x * x <= n).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::bulb_switch(3), 1);
        assert_eq!(Solution::bulb_switch(99999999), 9999);
    }
}
