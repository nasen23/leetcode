impl Solution {
    pub fn construct_arr(a: Vec<i32>) -> Vec<i32> {
        let mut b = vec![1; a.len()];
        for i in 1..a.len() {
            b[i] = b[i - 1] * a[i - 1];
        }
        let mut p = 1;
        for i in (1..a.len()).rev() {
            p *= a[i];
            b[i - 1] *= p;
        }
        b
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_times_arr() {
        assert_eq!(Vec::<i32>::new(), Solution::construct_arr(vec![]));
    }
}
