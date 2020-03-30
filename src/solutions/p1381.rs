use std::collections::VecDeque;

struct CustomStack {
    size: usize,
    deque: VecDeque<i32>,
}

impl CustomStack {
    fn new(max_size: i32) -> Self {
        Self {
            size: max_size as usize,
            deque: VecDeque::with_capacity(max_size as usize),
        }
    }

    fn push(&mut self, x: i32) {
        if self.deque.len() < self.size {
            self.deque.push_back(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.deque.pop_back().unwrap_or(-1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        for x in self.deque.iter_mut().take(k as usize) {
            *x += val;
        }
    }
}
