impl Solution {
    fn dfs(i: usize, j: usize, dp: &mut Vec<Vec<i32>>, matrix: &Vec<Vec<i32>>) -> i32 {
        if dp[i][j] != 0 {
            return dp[i][j];
        }
        let n = matrix.len();
        let m = matrix[0].len();
        dp[i][j] = 1;
        if i > 0 && matrix[i][j] < matrix[i-1][j] {
            dp[i][j] = std::cmp::max(dp[i][j], 1 + Solution::dfs(i-1, j, dp, matrix));
        }
        if j > 0 && matrix[i][j] < matrix[i][j-1] {
            dp[i][j] = std::cmp::max(dp[i][j], 1 + Solution::dfs(i, j-1, dp, matrix));
        }
        if i < n-1 && matrix[i][j] < matrix[i+1][j] {
            dp[i][j] = std::cmp::max(dp[i][j], 1 + Solution::dfs(i+1, j, dp, matrix));
        }
        if j < m-1 && matrix[i][j] < matrix[i][j+1] {
            dp[i][j] = std::cmp::max(dp[i][j], 1 + Solution::dfs(i, j+1, dp, matrix));
        }
        dp[i][j]
    }

    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut dp = (0..n).map(|_i| vec![0; m]).collect();
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                ans = std::cmp::max(ans, Solution::dfs(i, j, &mut dp, &matrix));
            }
        }
        ans
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::longest_increasing_path(vec![vec![3,4,5],vec![3,2,6],vec![2,2,1]]));
}