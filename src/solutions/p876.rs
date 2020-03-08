use crate::ListNode;

type Link = Option<Box<ListNode>>;

struct Solution;

impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // fast and slow pointers
        let mut fast = &mut head as *mut Link;
        let mut slow = fast;

        unsafe {
            let move_p = |mut x: *mut Link, k: i32| {
                for _ in 0..k {
                    if *x == None {
                        return x;
                    } else {
                        x = &mut (*x).as_mut().unwrap().next as *mut Link;
                    }
                }

                x
            };

            while *fast != None && (*fast).as_ref().unwrap().next != None {
                fast = move_p(fast, 2);
                slow = move_p(slow, 1);
            }

            (*slow).take()
        }
    }
}
