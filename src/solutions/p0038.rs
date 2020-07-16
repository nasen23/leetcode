impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut res = "1".to_owned();
        for _ in 1..n {
            res = {
                let mut new = String::new();
                let mut c = 0;
                let mut p = '0';
                for ch in res.chars() {
                    if ch == p {
                        c += 1;
                    } else {
                        if c > 0 {
                            new.push_str(&c.to_string());
                            new.push(p);
                        }
                        p = ch;
                        c = 1;
                    }
                }
                new.push_str(&c.to_string());
                new.push(p);
                new
            };
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_and_say() {
        assert_eq!(Solution::count_and_say(1), "1");
        assert_eq!(Solution::count_and_say(4), "1211");
        assert_eq!(Solution::count_and_say(5), "111221");
    }
}
