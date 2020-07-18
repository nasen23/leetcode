impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let (mut l, mut r) = (vec![0; n], vec![0; n]);
        for i in 1..n {
            l[i] = l[i - 1].max(height[i - 1]);
        }
        for i in (1..n).rev() {
            r[i - 1] = r[i].max(height[i]);
        }
        let mut res = 0;
        for i in 1..n {
            let level = l[i].min(r[i]);
            res += 0.max(level - height[i]);
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trapping_rain_water() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
}
