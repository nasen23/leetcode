use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;


struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;
static mut ANS: i32 = 0;

impl Solution {
    pub fn longest_univalue_path(root: Tree) -> i32 {
        arrow_length(&root);
        unsafe { ANS }
    }
}

/// Single line length.
fn arrow_length(node: &Tree) -> i32 {
    match node {
        Some(node) => {
            let node = node.borrow();
            let llen = arrow_length(&node.left);
            let rlen = arrow_length(&node.right);

            let (mut aleft, mut aright) = (0, 0);
            if let Some(lnode) = &node.left {
                if lnode.borrow().val == node.val {
                    aleft = llen + 1;
                }
            }
            if let Some(rnode) = &node.right {
                if rnode.borrow().val == node.val {
                    aright = rlen + 1;
                }
            }

            unsafe { ANS = std::cmp::max(ANS, aleft + aright) }
            std::cmp::max(aleft, aright)
        },
        None => 0,
    }
}

#[test]
fn simple_1() {
    let node = TreeNode::new(1);
    let tree = Some(Rc::new(RefCell::new(node)));
    assert_eq!(0, Solution::longest_univalue_path(tree));
}
