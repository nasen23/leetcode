use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k);
        for num in nums {
            heap.push(Reverse(num));
            if heap.len() > k {
                heap.pop();
            }
        }
        Self { k, heap }
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.k {
            self.heap.push(Reverse(val));
        } else if val > self.heap.peek().unwrap().0 {
            self.heap.push(Reverse(val));
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}
