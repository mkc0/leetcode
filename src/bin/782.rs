impl Solution {
    fn check_zip(z: usize, n: usize) -> Option<i32> {
        let one_cnt = (0..n).fold(0, |acc, i| {
            if ((1<<i) & z) > 0 {
                return acc + 1;
            }
            acc
        });
        let zero_cnt = n - one_cnt;
        if one_cnt == zero_cnt - 1 {
            return Solution::check_zip(((1<<n)-1) ^ z, n);
        }
        let need_one_cnt = (0..n).fold(0, |acc, i| {
            if i % 2 == 0 && ((1<<i) & z) == 0 {
                return acc + 1;
            }
            acc
        });
        if one_cnt == zero_cnt + 1 {
            return Some(need_one_cnt as i32);
        } else if one_cnt == zero_cnt {
            let need = std::cmp::min((n/2) - need_one_cnt, need_one_cnt);
            return Some(need as i32);
        }
        return None;
    }

    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let rowz: Vec<i32> = board
            .iter()
            .map(|row| row.iter().enumerate().fold(0, |acc, (i, x)| acc + (x << i)))
            .collect();
        let zero = rowz[0];
        let one = ((1<<n)-1) ^ zero;
        let mut ok = true;
        let boardz = rowz.iter().enumerate().fold(0, |acc, (i, x)| {
            if *x == one {
                return acc + (1 << i);
            } else if *x == zero {
                return acc;
            } else {
                ok = false;
                return acc;
            }
        });
        if !ok {
            return -1;
        }
        let mut ans = 0;
        if let Some(x) = Solution::check_zip(zero as usize, n) {
            ans += x;
        } else {
            return -1;
        }
        if let Some(x) = Solution::check_zip(boardz as usize, n) {
            ans += x;
        } else {
            return -1;
        }
        ans
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::moves_to_chessboard(vec![vec![0,1,1,0],vec![0,1,1,0],vec![1,0,0,1],vec![1,0,0,1]])
    );
    println!(
        "{}",
        Solution::moves_to_chessboard(vec![vec![1,1,0],vec![0,0,1],vec![0,0,1]])
    );
}
