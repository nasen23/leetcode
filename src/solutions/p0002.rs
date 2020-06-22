use crate::ListNode;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;
        let (mut l1, mut l2) = (l1, l2);
        let (mut l1_end, mut l2_end, mut carry) = (false, false, 0);
        loop {
            let lhs = match l1 {
                Some(node) => {
                    l1 = node.next;
                    node.val
                }
                None => {
                    l1_end = true;
                    0
                }
            };
            let rhs = match l2 {
                Some(node) => {
                    l2 = node.next;
                    node.val
                }
                None => {
                    l2_end = true;
                    0
                }
            };
            if l1_end && l2_end && carry == 0 {
                break head;
            }
            let res = lhs + rhs + carry;
            let val = if res < 10 {
                carry = 0;
                res
            } else {
                carry = 1;
                res - 10
            };
            *tail = Some(Box::new(ListNode::new(val)));
            tail = &mut tail.as_mut().unwrap().next;
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_numbers() {
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode::new(1))),
                Some(Box::new(ListNode::new(7)))
            ),
            Some(Box::new(ListNode::new(8)))
        );
    }
}
