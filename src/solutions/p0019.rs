impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut l: *const _ = &head;
        let mut d = head.as_mut().unwrap();
        let mut i = 0;
        while let Some(sl) = unsafe { &*l } {
            l = &sl.next;
            if i > n {
                d = d.next.as_mut().unwrap()
            }
            i += 1;
        }
        if i == n {
            return head.unwrap().next;
        }
        d.next = d.next.take().unwrap().next;
        head
    }
}

use crate::ListNode;
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linkedlist;

    #[test]
    fn remove_nth_back() {
        assert_eq!(
            Solution::remove_nth_from_end(linkedlist!(1, 2, 3, 4, 5), 2),
            linkedlist!(1, 2, 3, 5)
        );
    }
}
