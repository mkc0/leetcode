use std::collections::HashSet;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut scanned = HashSet::new();
        let mut scanned2 = HashSet::new();
        let mut zero_cnt = 0;
        for num in arr {
            if num == 0 {
                zero_cnt += 1;
                continue;
            }
            if num % 2 == 0 {
                let half = num / 2;
                if scanned.contains(&half) {
                    return true;
                }
            }
            let num2 = num * 2;
            if scanned.contains(&num2) {
                return true;
            }
            scanned.insert(num);
            scanned2.insert(num2);
        }
        return zero_cnt >= 2;
    }
}

struct Solution;

fn main() {
    Solution::check_if_exist(vec![1, 2, 3]);
}
