struct Solution;

use crate::ListNode;

use std::collections::VecDeque;

type List = Option<Box<ListNode>>;

impl Solution {
    pub fn add_two_numbers(mut l1: List, mut l2: List) -> List {
        let mut s1 = VecDeque::with_capacity(16);
        let mut s2 = VecDeque::with_capacity(16);

        while l1.is_some() {
            let x = l1.unwrap();
            s1.push_back(x.val);
            l1 = x.next;
        }
        while l2.is_some() {
            let x = l2.unwrap();
            s2.push_back(x.val);
            l2 = x.next;
        }

        let mut carry = 0;
        let mut head = None;
        while !s1.is_empty() || !s2.is_empty() || carry > 0 {
            let sum = carry + s1.pop_back().unwrap_or(0) + s2.pop_back().unwrap_or(0);
            let mut node = ListNode::new(sum % 10);
            node.next = head;
            head = Some(Box::new(node));
            carry = sum / 10;
        }

        head
    }
}
