fn a2u(x: u8) -> usize {
    (x - b'a') as usize
}

impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        let s = s.as_bytes();
        let (mut l, mut r) = (vec![s.len(); 26], vec![0; 26]);
        let mut c = vec![0; 26];
        for i in 0..s.len() {
            let x = a2u(s[i]);
            l[x] = l[x].min(i);
            r[x] = r[x].max(i);
            c[x] += 1;
        }
        let mut ranges = vec![];
        for i in (0..26).filter(|&i| l[i] != s.len()) {
            let (mut x, mut y, mut tot) = (l[i], r[i], c[i]);
            let mut u = vec![false; 26];
            u[i] = true;
            while y - x + 1 != tot {
                for k in (x + 1..y).map(|k| a2u(s[k])).filter(|&ch| l[ch] != s.len()) {
                    if u[k] {
                        continue;
                    }
                    x = x.min(l[k]);
                    y = y.max(r[k]);
                    tot += c[k];
                    u[k] = true;
                }
            }
            ranges.push((x, y));
        }
        ranges.sort_by(|t1, t2| (t1.1 - t1.0).cmp(&(t2.1 - t2.0)));
        let mut ok = vec![false; ranges.len()];
        let mut res = vec![];
        for i in 0..ranges.len() {
            if ok[i] {
                continue;
            }
            let (x, y) = ranges[i];
            for j in 0..ranges.len() {
                let (l, r) = ranges[j];
                if l <= x && y <= r {
                    ok[j] = true;
                }
            }
            res.push(String::from_utf8(s[x..=y].to_vec()).unwrap());
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_str;

    #[test]
    fn maximum_number_of_nonoverlapping_substrings() {
        assert_eq!(
            Solution::max_num_of_substrings("adefaddaccc".into()),
            vec_str!["e", "f", "ccc"]
        );
        assert_eq!(
            Solution::max_num_of_substrings("abab".into()),
            vec_str!["abab"]
        );
        assert_eq!(
            Solution::max_num_of_substrings("cabcccbaa".into()),
            vec_str!["cabcccbaa"]
        );
    }
}
