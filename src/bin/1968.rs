impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut snums = nums.to_vec();
        snums.sort();
        let len = nums.len();
        let mut lidx = 0usize;
        let mut ridx = len - 1usize;
        let mut res = vec![0i32; len];
        let mut lorr = true;
        for i in 0..len {
            if lorr {
                res[i] = snums[lidx];
                lidx += 1;
            } else {
                res[i] = snums[ridx];
                ridx -= 1;
            }
            lorr = !lorr;
        }
        res
    }
}

struct Solution;

fn main() {
    Solution::rearrange_array(vec![0, 1]);
}
