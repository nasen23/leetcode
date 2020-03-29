struct Solution;

impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        let n = n.parse::<u64>().unwrap();
        let mut m = (n as f64).log2() as u32;

        let res = loop {
            let k = (n as f64).powf(1.0 / m as f64);
            let k = if k as u64 == k.ceil() as u64 {
                k as u64 - 1
            } else {
                k as u64
            };
            if k == 1 {
                m -= 1;
                continue;
            }
            if n - (n - 1) / k == k.pow(m) {
                break k;
            }
            m -= 1;
            if m == 0 {
                break n - 1;
            }
        };

        res.to_string()
    }
}

#[test]
fn case1() {
    assert_eq!(Solution::smallest_good_base("13".to_string()), "3");
}

#[test]
fn case2() {
    assert_eq!(
        Solution::smallest_good_base("1000000000000000000".to_string()),
        "999999999999999999"
    );
}

#[test]
fn case3() {
    assert_eq!(Solution::smallest_good_base("3".to_string()), "2");
}

#[test]
fn case4() {
    assert_eq!(
        Solution::smallest_good_base("2251799813685247".to_string()),
        "2"
    );
}
