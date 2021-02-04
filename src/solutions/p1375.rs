impl Solution {
    pub fn num_times_all_blue(light: Vec<i32>) -> i32 {
        let mut max_idx = 0;
        let mut c = 0;
        for (i, l) in light.into_iter().enumerate() {
            max_idx = l.max(max_idx);
            if max_idx == (i + 1) as i32 {
                c += 1;
            }
        }
        c
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bulb_switcher_iii() {
        assert_eq!(3, Solution::num_times_all_blue(vec![2, 1, 3, 5, 4]));
    }
}
