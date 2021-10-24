use std::collections::BTreeSet;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut mn_num = std::i32::MAX;
        let mns: Vec<i32> = nums
            .iter()
            .map(|num| {
                mn_num = std::cmp::min(mn_num, *num);
                mn_num
            })
            .collect();
        let mut set = BTreeSet::new();
        let n = nums.len();
        for i in (0..n).rev() {
            use std::ops::Bound::*;
            let mut rng = set.range((Unbounded, Excluded(nums[i])));
            if let Some(&q) = rng.next_back() {
                if q > mns[i]  {
                    return true;
                }
            }
            set.insert(nums[i]);
        }
        false
    }
}

struct Solution;

fn main() {
    Solution::find132pattern(vec![1, 2, 3]);
}