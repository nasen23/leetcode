use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

struct KVPair {
    value: i32,
    index: usize,
}

impl KVPair {
    fn new(value: i32, index: usize) -> Self {
        Self { value, index }
    }
}

impl PartialEq for KVPair {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn ne(&self, other: &Self) -> bool {
        self.value != other.value
    }
}

impl PartialOrd for KVPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl Eq for KVPair {}

impl Ord for KVPair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        let mut heads = Vec::new();
        for list in lists.iter() {
            heads.push(list);
        }
        for (i, head) in heads.iter_mut().enumerate() {
            if let Some(node) = head {
                heap.push(Reverse(KVPair::new(node.val, i)));
                *head = &node.next;
            }
        }
        let mut merged_list: Option<Box<ListNode>> = None;
        let mut list_tail = &mut merged_list;
        while let Some(kv_pair) = heap.pop() {
            let new_node = Box::new(ListNode::new(kv_pair.0.value));
            if let Some(node) = heads[kv_pair.0.index] {
                heap.push(Reverse(KVPair::new(node.val, kv_pair.0.index)));
                heads[kv_pair.0.index] = &node.next;
            }
            if let Some(tail) = list_tail {
                tail.next = Some(new_node);
                list_tail = &mut tail.next;
            } else {
                *list_tail = Some(new_node);
            }
        }
        merged_list
    }
}

use crate::ListNode;
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linkedlist;

    #[test]
    fn merge_k_sorted_lists() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                linkedlist!(1, 4, 5),
                linkedlist!(1, 3, 4),
                linkedlist!(2, 6)
            ]),
            linkedlist!(1, 1, 2, 3, 4, 4, 5, 6)
        );
    }
}
