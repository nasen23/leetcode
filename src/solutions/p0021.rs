use crate::ListNode;

type List = Option<Box<ListNode>>;

impl Solution {
    pub fn merge_two_lists(mut l1: List, mut l2: List) -> List {
        let mut res = ListNode::new(0);
        let mut p = &mut res;

        while l1.is_some() && l2.is_some() {
            let (n1, n2) = (l1.as_deref_mut().unwrap(), l2.as_deref_mut().unwrap());
            if n1.val < n2.val {
                let next = n1.next.take();
                p.next = l1.take();
                l1 = next;
            } else {
                let next = n2.next.take();
                p.next = l2.take();
                l2 = next;
            }
            p = p.next.as_deref_mut().unwrap();
        }
        p.next = l1.or(l2);

        res.next
    }
}

struct Solution;
