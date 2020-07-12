impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x = x.to_string();
        x.bytes().eq(x.bytes().rev())
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindrome_number() {
        assert!(Solution::is_palindrome(121));
        assert!(Solution::is_palindrome(121));
    }
}
