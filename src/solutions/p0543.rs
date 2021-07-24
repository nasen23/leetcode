use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

static mut MAX_LENGTH: i32 = 0;

impl Solution {
    pub fn diameter_of_binary_tree(root: Tree) -> i32 {
        fn max_depth(node: &Tree) -> i32 {
            match node {
                Some(node) => {
                    let node = node.borrow();
                    let left = max_depth(&node.left);
                    let right = max_depth(&node.right);
                    unsafe {
                        if left + right > MAX_LENGTH {
                            MAX_LENGTH = left + right;
                        }
                    }
                    1 + std::cmp::max(left, right)
                }
                None => 0,
            }
        }

        max_depth(&root);
        unsafe { MAX_LENGTH }
    }
}

#[test]
fn simple_none() {
    let root = None;
    assert_eq!(Solution::diameter_of_binary_tree(root), 0);
}
