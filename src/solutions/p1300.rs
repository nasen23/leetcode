impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let mut arr = arr;
        arr.sort();
        let mut s = 0;
        for i in 0..arr.len() {
            let ss = s + (arr.len() - i) as i32 * arr[i];
            if ss > target {
                let val = (target - s) / (arr.len() - i) as i32;
                let d1 = ((s + (arr.len() - i) as i32 * val) - target).abs();
                let d2 = ((s + (arr.len() - i) as i32 * (val + 1)) - target).abs();
                return if d1 > d2 { val + 1 } else { val };
            }
            s += arr[i];
        }

        arr[arr.len() - 1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_best_value() {
        assert_eq!(Solution::find_best_value(vec![4, 9, 3], 10), 3);
        assert_eq!(Solution::find_best_value(vec![2, 3, 5], 10), 5);
        assert_eq!(
            Solution::find_best_value(vec![60864, 25176, 27249, 21296, 20204], 56803),
            11361
        );
    }
}
