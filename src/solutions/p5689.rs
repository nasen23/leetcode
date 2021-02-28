impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let idx = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => unreachable!(),
        };
        items
            .into_iter()
            .filter(|item| item[idx] == rule_value)
            .count() as i32
    }
}

struct Solution;
