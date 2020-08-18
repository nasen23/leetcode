use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if head.is_none() {
            return None;
        }
        let (mut slow, mut fast) = (&head, &head);
        let move_p = |p: &mut &Option<Box<ListNode>>, s: i32| {
            for _ in 0..s {
                if let Some(node) = p.as_ref() {
                    *p = &node.next;
                }
            }
        };
        move_p(&mut fast, 2);
        while fast.is_some() {
            move_p(&mut slow, 1);
            move_p(&mut fast, 2);
        }
        let back = unsafe {
            (&mut *(slow as *const _ as *mut Option<Box<ListNode>>))
                .take()
                .unwrap()
        };
        let left = Solution::sorted_list_to_bst(head);
        let right = Solution::sorted_list_to_bst(back.next);
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: back.val,
            left,
            right,
        })));
        node
    }
}

struct Solution;
use crate::{ListNode, TreeNode};
