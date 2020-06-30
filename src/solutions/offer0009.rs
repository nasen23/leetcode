use std::collections::VecDeque;

struct CQueue {
    a: VecDeque<i32>,
    b: VecDeque<i32>,
}

impl CQueue {
    fn new() -> Self {
        Self {
            a: VecDeque::new(),
            b: VecDeque::new(),
        }
    }

    fn append_tail(&mut self, value: i32) {
        self.a.push_back(value);
    }

    fn delete_head(&mut self) -> i32 {
        self.b
            .pop_back()
            .or_else(|| {
                while let Some(value) = self.a.pop_back() {
                    self.b.push_back(value);
                }
                self.b.pop_back()
            })
            .unwrap_or(-1)
    }
}
