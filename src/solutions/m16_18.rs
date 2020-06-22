impl Solution {
    pub fn pattern_matching(pattern: String, value: String) -> bool {
        let pattern = pattern.as_bytes();
        let value = value.as_bytes();
        let pp = pattern.len();
        let ss = value.len();
        if pp == 0 {
            return ss == 0;
        }
        let (a, b) = pattern.iter().fold((0, 0), |(a, b), by| match by {
            b'a' => (a + 1, b),
            b'b' => (a, b + 1),
            _ => unreachable!(),
        });
        if a != 0 && b != 0 && ss == 0 {
            return false;
        }
        macro_rules! check {
            ($x:expr, $y:expr) => {{
                if $y == 0 {
                    if ss % $x != 0 {
                        return false;
                    }
                    let c = ss / $x;
                    let slice = &value[0..c];
                    for i in 0..$x {
                        for j in 0..c {
                            if slice[j] != value[i * c + j] {
                                return false;
                            }
                        }
                    }
                    return true;
                }
            }};
        }
        check!(a, b);
        check!(b, a);
        'outer: for i in 0..=ss / a {
            // length of a
            if (ss - i * a) % b != 0 {
                continue;
            }
            let j = (ss - i * a) / b; // length of b
                                      // slice of a and b
            let (mut sa, mut sb): (Option<&[u8]>, Option<&[u8]>) = (None, None);
            let mut sp = 0;
            for by in pattern.iter() {
                if let Some(aa) = sa {
                    if let Some(bb) = sb {
                        if aa == bb {
                            continue 'outer;
                        }
                    }
                }
                match by {
                    b'a' => match sa {
                        Some(si) => {
                            for k in 0..i {
                                if si[k] != value[sp + k] {
                                    continue 'outer;
                                }
                            }
                            sp += i;
                        }
                        None => {
                            sa = Some(&value[sp..sp + i]);
                            sp += i;
                        }
                    },
                    b'b' => match sb {
                        Some(si) => {
                            for k in 0..j {
                                if si[k] != value[sp + k] {
                                    continue 'outer;
                                }
                            }
                            sp += j;
                        }
                        None => {
                            sb = Some(&value[sp..sp + j]);
                            sp += j;
                        }
                    },
                    _ => unreachable!(),
                }
            }
            return true;
        }
        false
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_matching() {
        assert!(!Solution::pattern_matching(
            "aaaa".into(),
            "dogcatcatdog".into()
        ));
        assert!(Solution::pattern_matching(
            "abba".into(),
            "dogcatcatdog".into()
        ));
        assert!(!Solution::pattern_matching(
            "abba".into(),
            "dogcatcatfish".into()
        ));
        assert!(Solution::pattern_matching(
            "abba".into(),
            "dogdogdogdog".into()
        ));
        assert!(Solution::pattern_matching("bbb".into(), "xxxxxx".into()));
        assert!(Solution::pattern_matching(
            "bbbbbbbbbbbbbbabbbbb".into(),
            "ppppppppppppppjsftcleifftfthiehjiheyqkhjfkyfckbtwbelfcgihlrfkrwireflijkjyppppg".into()
        ));
    }
}
