impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = head;
        let mut res: Option<Box<ListNode>> = None;
        let mut tail = &mut res;
        while let Some(mut n1) = p {
            p = n1.next.take();
            if let Some(mut n2) = p {
                p = n2.next.take();
                n2.next = Some(n1);
                if let Some(tail_box) = tail {
                    tail_box.next = Some(n2);
                    tail = &mut tail_box.next.as_mut().unwrap().next;
                } else {
                    *tail = Some(n2);
                    tail = &mut tail.as_mut().unwrap().next;
                }
            } else {
                if let Some(tail_box) = tail {
                    tail_box.next = Some(n1);
                } else {
                    *tail = Some(n1);
                }
            }
        }
        res
    }
}

use crate::ListNode;
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linkedlist;

    #[test]
    fn swap_nodes_in_pairs() {
        assert_eq!(
            Solution::swap_pairs(linkedlist!(1, 2, 3, 4)),
            linkedlist!(2, 1, 4, 3)
        );
        assert_eq!(
            Solution::swap_pairs(linkedlist!(1, 2, 3, 4, 5)),
            linkedlist!(2, 1, 4, 3, 5)
        );
    }
}
