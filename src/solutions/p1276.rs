impl Solution {
    pub fn num_of_burgers(a: i32, b: i32) -> Vec<i32> {
        if (a - 2 * b) % 2 == 1 {
            return vec![];
        }
        let x = a / 2 - b;
        let y = b - x;
        if x >= 0 && y >= 0 {
            vec![x, y]
        } else {
            vec![]
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_of_burgers() {
        assert_eq!(Solution::num_of_burgers(16, 7), vec![1, 6]);
        assert_eq!(Solution::num_of_burgers(17, 4), vec![]);
        assert_eq!(Solution::num_of_burgers(4, 17), vec![]);
        assert_eq!(Solution::num_of_burgers(4, 17), vec![]);
        assert_eq!(Solution::num_of_burgers(0, 0), vec![0, 0]);
        assert_eq!(Solution::num_of_burgers(2, 1), vec![0, 1]);
    }
}
