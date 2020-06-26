use crate::ListNode;
use std::collections::HashSet;

impl Solution {
    pub fn remove_duplicate_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut set = HashSet::new();
        let mut p = head;
        let mut head = None;
        let mut q = &mut head;
        while let Some(node) = p {
            if set.insert(node.val) {
                *q = Some(Box::new(ListNode::new(node.val)));
                q = &mut q.as_mut().unwrap().next;
            }
            p = node.next;
        }
        head
    }
}

struct Solution;
