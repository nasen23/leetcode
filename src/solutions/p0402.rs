struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut num = num;
        if num.as_bytes().len() == k as usize {
            return "0".into();
        }

        for _ in 0..k {
            let idx = num
                .as_bytes()
                .windows(2)
                .enumerate()
                .find(|(_, x)| x[0] > x[1])
                .map(|x| x.0)
                .unwrap_or(num.as_bytes().len() - 1);
            num.remove(idx);
            num = num.trim_start_matches('0').to_owned();
            if num.is_empty() {
                return "0".into();
            }
        }

        num
    }
}

#[test]
fn test() {
    assert_eq!(Solution::remove_kdigits("1432219".into(), 3), "1219");
    assert_eq!(Solution::remove_kdigits("10200".into(), 1), "200");
    assert_eq!(Solution::remove_kdigits("10".into(), 2), "0");
    assert_eq!(Solution::remove_kdigits("112".into(), 1), "11");
    assert_eq!(Solution::remove_kdigits("5337".into(), 2), "33");
}
