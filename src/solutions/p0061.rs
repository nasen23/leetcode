impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 {
            return head;
        }
        let mut p = &head;
        let mut len = 0;
        while let Some(node) = p {
            p = &node.next;
            len += 1;
        }
        let x = len - (k % len);
        if len == 1 || len == x {
            return head;
        }
        let mut p = &mut head;
        for _ in 0..x {
            p = &mut p.as_mut().unwrap().next;
        }
        let mut remain = p.take();
        let mut p = &mut remain;
        while let Some(node) = p {
            p = &mut node.next;
        }
        *p = head;
        remain
    }
}

use crate::ListNode;
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linkedlist;

    #[test]
    fn rotate_list() {
        assert_eq!(
            Solution::rotate_right(linkedlist!(0, 1, 2), 4),
            linkedlist!(2, 0, 1)
        );
    }
}
