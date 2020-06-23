impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut cy = false;
        let (mut iter_a, mut iter_b) = (a.chars().rev(), b.chars().rev());
        let mut res = String::new();
        loop {
            match (iter_a.next(), iter_b.next()) {
                (Some(aa), Some(bb)) => {
                    let (val, ncy) = sum(aa, bb, cy);
                    cy = ncy;
                    res.push(val);
                }
                (Some(aa), None) => {
                    let (val, ncy) = sum(aa, '0', cy);
                    cy = ncy;
                    res.push(val);
                }
                (None, Some(bb)) => {
                    let (val, ncy) = sum('0', bb, cy);
                    cy = ncy;
                    res.push(val);
                }
                (None, None) => {
                    if cy {
                        res.push('1');
                    }
                    break;
                }
            }
        }

        res.chars().rev().collect()
    }
}

#[inline]
fn sum(a: char, b: char, cy: bool) -> (char, bool) {
    match (a, b, cy) {
        ('0', '0', false) => ('0', false),
        ('0', '0', true) | ('0', '1', false) | ('1', '0', false) => ('1', false),
        ('1', '0', true) | ('0', '1', true) | ('1', '1', false) => ('0', true),
        _ => ('1', true),
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_binary() {
        assert_eq!(Solution::add_binary("1".into(), "1".into()), "10");
        assert_eq!(Solution::add_binary("11".into(), "1".into()), "100");
        assert_eq!(Solution::add_binary("1010".into(), "1011".into()), "10101");
    }
}
