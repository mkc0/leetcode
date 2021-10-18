impl Solution {
    fn sort_and_count(sums: &mut [i64], lower: i64, upper: i64) -> i32 {
        let len = sums.len();
        if len == 1 {
            if sums[0] <= upper && sums[0] >= lower {
                return 1;
            } else {
                return 0;
            }
        }
        // count
        let mut count = 0_i32;
        let mid = len / 2;
        count += Solution::sort_and_count(&mut sums[..mid], lower, upper);
        count += Solution::sort_and_count(&mut sums[mid..], lower, upper);
        let mut lower_i = mid;
        let mut upper_i = mid;
        for i in 0..mid {
            while upper_i != len && sums[upper_i] - sums[i] <= upper {
                upper_i += 1;
            }
            while lower_i != len && sums[lower_i] - sums[i] < lower {
                lower_i += 1;
            }
            count += (upper_i - lower_i) as i32;
        }
        // println!("{:?} {}", sums, count);
        // sort
        let copied = sums.to_vec();
        let mut index = 0;
        let mut left_i = 0;
        let mut right_i = mid;
        while left_i != mid || right_i != len {
            if left_i == mid || (right_i != len && copied[right_i] < copied[left_i]) {
                sums[index] = copied[right_i];
                right_i += 1;
                index += 1;
            } else {
                sums[index] = copied[left_i];
                left_i += 1;
                index += 1;
            }
        }
        count
    }

    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut sum = 0_i64;
        let mut sums: Vec<i64> = nums
            .iter()
            .map(|num| {
                sum += *num as i64;
                sum
            })
            .collect();
        if lower > upper {
            return 0;
        }
        Solution::sort_and_count(&mut sums[..], lower as i64, upper as i64)
    }
}

struct Solution;

fn main() {
    let a = vec![2147483647, -2147483648, -1, 0];
    println!("{}", Solution::count_range_sum(a, -1, 0));
}
