impl Solution {
    pub fn is_number(s: String) -> bool {
        let s = s.trim().split('e').collect::<Vec<_>>();
        match s[..] {
            [float] => is_float(float),
            [sig, exp] => is_float(sig) && is_integer(exp),
            _ => false,
        }
    }
}

fn is_float(s: &str) -> bool {
    may_have_sign_ahead(is_unsigned_float, s)
}

fn is_integer(s: &str) -> bool {
    may_have_sign_ahead(is_unsigned_integer, s)
}

fn is_unsigned_integer(s: &str) -> bool {
    if s.is_empty() {
        false
    } else {
        is_unsigned_integer_or_empty(s)
    }
}

fn is_unsigned_integer_or_empty(s: &str) -> bool {
    for ch in s.chars() {
        match ch {
            '0'..='9' => {}
            _ => return false,
        }
    }
    true
}

fn is_unsigned_float(s: &str) -> bool {
    let s = s.split('.').collect::<Vec<_>>();
    match s[..] {
        [integer] => is_unsigned_integer(integer),
        [int_part, float_part] => {
            (!int_part.is_empty() || !float_part.is_empty())
                && is_unsigned_integer_or_empty(int_part)
                && is_unsigned_integer_or_empty(float_part)
        }
        _ => false,
    }
}

fn may_have_sign_ahead(predicate: impl Fn(&str) -> bool, s: &str) -> bool {
    match s.chars().nth(0) {
        Some('+') | Some('-') => predicate(&s[1..]),
        _ => predicate(s),
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_number() {
        assert!(Solution::is_number("0".into()));
        assert!(Solution::is_number("0.1".into()));
        assert!(!Solution::is_number("1 a".into()));
    }
}
