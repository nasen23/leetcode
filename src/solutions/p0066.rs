impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut c = false;
        for i in (0..digits.len()).rev() {
            if digits[i] == 9 {
                digits[i] = 0;
                if i == 0 {
                    c = true;
                }
            } else {
                digits[i] += 1;
                break;
            }
        }
        if c {
            digits.insert(0, 1);
        }
        digits
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_one() {
        assert_eq!(Solution::plus_one(vec![1, 2]), vec![1, 3]);
        assert_eq!(Solution::plus_one(vec![9, 9]), vec![1, 0, 0]);
    }
}
