impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        if s.len() < 4 || s.len() > 12 {
            return vec![];
        }
        let mut ret = vec![vec![s.as_str()]];
        for i in 0..3 {
            let mut tmp = vec![];
            while let Some(v) = ret.pop() {
                let s = v[i];
                for j in 1..=3.min(s.len() + i - 3) {
                    let mut v = v[..i].to_vec();
                    let (n, remain) = s.split_at(j);
                    if n != "0" && (n.starts_with('0') || n.parse::<u8>().is_err()) {
                        continue;
                    }
                    if i == 2 {
                        if remain != "0"
                            && (remain.starts_with('0') || remain.parse::<u8>().is_err())
                        {
                            continue;
                        }
                    }
                    if remain.len() > (3 - i) * 3 {
                        continue;
                    }
                    v.push(n);
                    v.push(remain);
                    tmp.push(v);
                }
            }
            ret = tmp;
        }
        ret.into_iter().map(|v| v.join(".")).collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_str;

    #[test]
    fn restore_ip_addr() {
        assert_eq!(
            Solution::restore_ip_addresses("25525511135".into()),
            vec_str!["255.255.11.135", "255.255.111.35"]
        );
        assert_eq!(
            Solution::restore_ip_addresses("010010".into()),
            vec_str!["0.100.1.0", "0.10.0.10"]
        );
    }
}
