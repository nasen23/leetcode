use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn merge_trees(
        t1: Option<Rc<RefCell<TreeNode>>>,
        t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (t1, t2) {
            (None, None) => None,
            (Some(t), None) | (None, Some(t)) => {
                let t = Rc::try_unwrap(t).unwrap().into_inner();
                let l = Solution::merge_trees(t.left, None);
                let r = Solution::merge_trees(t.right, None);
                Some(Rc::new(RefCell::new(TreeNode {
                    val: t.val,
                    left: l,
                    right: r,
                })))
            }
            (Some(t1), Some(t2)) => {
                let t1 = Rc::try_unwrap(t1).unwrap().into_inner();
                let t2 = Rc::try_unwrap(t2).unwrap().into_inner();
                let l = Solution::merge_trees(t1.left, t2.left);
                let r = Solution::merge_trees(t1.right, t2.right);
                Some(Rc::new(RefCell::new(TreeNode {
                    val: t1.val + t2.val,
                    left: l,
                    right: r,
                })))
            }
        }
    }
}

use crate::TreeNode;
struct Solution;
