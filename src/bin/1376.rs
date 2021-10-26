impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let n = n as usize;
        let head_id = head_id as usize;
        let mut dp = vec![-1i32; n];
        for i in 0..n {
            let mut j = i;
            if dp[i] > -1 {
                continue;
            }
            dp[i] = 0;
            loop {
                if j == head_id {
                    break;
                } else {
                    j = manager[j] as usize;
                    dp[i] += inform_time[j];
                    if dp[j] > -1 {
                        dp[i] += dp[j];
                        break;
                    }
                }
            }
            let mut t = dp[i];
            j = i;
            loop {
                if j == head_id {
                    break;
                } else {
                    j = manager[j] as usize;
                    if dp[j] > -1 {
                        break;
                    }
                    t -= inform_time[j];
                    dp[j] = t;
                }
            }
        }
        let mut ans = std::i32::MIN;
        for i in 0..n {
            ans = std::cmp::max(ans, dp[i]);
        }
        ans
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::num_of_minutes(1, 0, vec![-1], vec![0]));
    println!("{}", Solution::num_of_minutes(7, 6, vec![1,2,3,4,5,6,-1], vec![0,6,5,4,3,2,1]));
}
