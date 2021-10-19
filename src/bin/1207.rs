use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut cnt_map = HashMap::new();
        for num in arr {
            match cnt_map.get(&num) {
                Some(&t) => cnt_map.insert(num, t + 1),
                None => cnt_map.insert(num, 1),
            };
        }
        let mut cnt_set = HashSet::new();
        for &v in cnt_map.values() {
            if !cnt_set.insert(v) {
                return false;
            }
        }
        return true;
    }
}

struct Solution;

fn main() {
    Solution::unique_occurrences(vec![1, 2, 3]);
}
