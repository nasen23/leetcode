impl Solution {
    pub fn to_hex(num: i32) -> String {
        format!("{:x}", num)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn convert_a_number_to_hexadecimal() {
        assert_eq!(Solution::to_hex(26), "1a");
    }
}
