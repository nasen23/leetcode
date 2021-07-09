impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let mut d = vec![];
        for envelope in envelopes {
            if d.is_empty() || envelope[1] > *d.last().unwrap() {
                d.push(envelope[1]);
            } else {
                let (mut l, mut r) = (0, d.len());
                let mut pos = 0;
                while l < r {
                    let mid = (l + r) / 2;
                    if d[mid] < envelope[1] {
                        l = mid + 1;
                    } else {
                        pos = mid;
                        r = mid;
                    }
                }
                d[pos] = envelope[1];
            }
        }
        d.len() as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn russian_roll_envelops() {
        assert_eq!(
            3,
            Solution::max_envelopes(vec_vec![[5, 4], [6, 4], [6, 7], [2, 3]])
        );
    }
}
