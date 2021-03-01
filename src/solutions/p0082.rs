use crate::ListNode;

use std::collections::BTreeMap;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = head;
        let mut map = BTreeMap::new();
        while let Some(node) = p {
            let count = map.entry(node.val).or_insert(0);
            *count += 1;
            p = node.next;
        }
        let mut head = ListNode::new(0);
        let p = &mut head;
        map.into_iter()
            .filter(|(_, v)| *v == 1)
            .map(|(k, _)| k)
            .fold(p, |mut p, k| {
                p.next = Some(Box::new(ListNode::new(k)));
                p = p.next.as_mut().unwrap().as_mut();
                p
            });
        head.next
    }
}

struct Solution;
