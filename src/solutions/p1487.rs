use std::cell::Cell;
use std::collections::HashMap;

impl Solution {
    pub fn get_folder_names(mut names: Vec<String>) -> Vec<String> {
        let mut map: HashMap<String, Cell<i32>> = HashMap::new();
        for name in names.iter_mut() {
            match map.get(name) {
                Some(c) => loop {
                    c.set(c.get() + 1);
                    let tmp = format!("{}({})", name, c.get());
                    if map.contains_key(&tmp) {
                        continue;
                    }
                    *name = tmp;
                    map.insert(name.clone(), Cell::new(0));
                    break;
                },
                None => {
                    map.insert(name.clone(), Cell::new(0));
                }
            }
        }
        names
    }
}

struct Solution;
