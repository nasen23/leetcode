use crate::ListNode;

impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut tail = head.clone();
        let mut i = 0;
        while tail.is_some() && i < k {
            tail = tail.as_ref().unwrap().next.clone();
            i += 1;
        }
        if i == k {
            tail = Solution::reverse_k_group(tail, k);
            for _ in 0..k {
                let tmp = head.as_ref().unwrap().next.clone();
                head.as_mut().unwrap().next = tail;
                tail = head;
                head = tmp;
            }
            head = tail;
        }
        head
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::linkedlist;

    #[test]
    fn reverse_k_group() {
        assert_eq!(
            Solution::reverse_k_group(linkedlist![1, 2, 3, 4], 2),
            linkedlist![2, 1, 4, 3]
        );
    }
}
