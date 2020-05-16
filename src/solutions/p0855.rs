use std::collections::BTreeSet;

struct ExamRoom {
    students: BTreeSet<i32>,
    n: i32,
}

impl ExamRoom {
    fn new(n: i32) -> Self {
        Self {
            students: BTreeSet::new(),
            n,
        }
    }

    fn seat(&mut self) -> i32 {
        if self.students.is_empty() {
            self.students.insert(0);
            0
        } else {
            let (mut s, mut e) = (0, 0);
            for &pos in self.students.iter() {
                if (pos - e) / 2 > (e - s) / 2 {
                    s = e;
                    e = pos;
                }
            }
            let mut pos = s + (e - s) / 2;
            if self.n - 1 - self.students.last().unwrap() > e - s {
                pos = self.n - 1;
            }
            self.students.insert(pos);
            pos
        }
    }

    fn leave(&mut self, p: i32) {
        self.students.remove(&p);
    }
}
