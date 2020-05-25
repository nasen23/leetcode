use std::collections::HashMap;
use std::ptr;

struct LRUEntry {
    key: i32,
    val: i32,
    prev: *mut LRUEntry,
    next: *mut LRUEntry,
}

impl LRUEntry {
    fn new(key: i32, val: i32) -> Self {
        Self {
            key,
            val,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }

    fn new_uninit() -> Self {
        Self {
            key: 0,
            val: 0,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}

struct LRUCache {
    ki: HashMap<i32, Box<LRUEntry>>,
    cap: usize,
    head: *mut LRUEntry,
    tail: *mut LRUEntry,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let cache = Self {
            ki: HashMap::new(),
            cap: capacity as usize,
            head: Box::into_raw(Box::new(LRUEntry::new_uninit())),
            tail: Box::into_raw(Box::new(LRUEntry::new_uninit())),
        };

        unsafe {
            (*cache.head).next = cache.tail;
            (*cache.tail).prev = cache.head;
        }

        cache
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.ki.get_mut(&key) {
            Some(node) => {
                let p: *mut LRUEntry = &mut **node;
                self.detach(p);
                self.attach(p);
                unsafe { (*p).val }
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let p = self.ki.get_mut(&key).map(|node| {
            let p: *mut LRUEntry = &mut **node;
            p
        });

        match p {
            Some(p) => {
                unsafe {
                    (*p).val = value;
                }
                self.detach(p);
                self.attach(p);
            }
            None => {
                let mut node = if self.ki.len() == self.cap {
                    let old_key = unsafe { (*(*self.tail).prev).key };
                    let mut old_node = self.ki.remove(&old_key).unwrap();

                    old_node.key = key;
                    old_node.val = value;

                    let p: *mut _ = &mut *old_node;
                    self.detach(p);

                    old_node
                } else {
                    Box::new(LRUEntry::new(key, value))
                };
                let p: *mut _ = &mut *node;
                self.attach(p);

                self.ki.insert(node.key, node);
            }
        }
    }

    fn detach(&mut self, node: *mut LRUEntry) {
        unsafe {
            (*(*node).prev).next = (*node).next;
            (*(*node).next).prev = (*node).prev;
        }
    }

    fn attach(&mut self, node: *mut LRUEntry) {
        unsafe {
            (*node).next = (*self.head).next;
            (*node).prev = self.head;
            (*self.head).next = node;
            (*(*node).next).prev = node;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LRUCache;

    #[test]
    fn lru_cache() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}
