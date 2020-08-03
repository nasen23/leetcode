use std::collections::LinkedList;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut num2 = num2.bytes().rev();
        let mut list = LinkedList::new();
        let mut carry = 0;
        for ch in num1.bytes().rev() {
            let ch2 = num2.next();
            let mut t = ch - b'0' + ch2.unwrap_or(b'0') - b'0' + carry;
            if t > 9 {
                t -= 10;
                carry = 1;
            } else {
                carry = 0;
            }
            list.push_front((t + b'0') as char);
        }
        for ch in num2 {
            let mut t = ch - b'0' + carry;
            if t > 9 {
                t -= 10;
                carry = 1;
            } else {
                carry = 0;
            }
            list.push_front((t + b'0') as char);
        }
        if carry == 1 {
            list.push_front('1');
        }
        list.into_iter().collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_strings() {
        assert_eq!(Solution::add_strings("999".into(), "999".into()), "1998");
    }
}
