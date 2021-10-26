impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut row = vec![1i32];
        let res:Vec<Vec<i32>> = (0..num_rows as usize).map(
            |n| {
                let mut added_row = row.to_vec();
                if n > 0 {
                    added_row.push(1);
                    for i in 1..n {
                        added_row[i] += row[i-1];
                    }
                }
                row = added_row.to_vec();
                added_row
            }).collect();
        res
    }
}

struct Solution;

fn main() {
    Solution::generate(1);
}
