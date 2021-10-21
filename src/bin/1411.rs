const MODULO: i64 = 1_000_000_007_i64;

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let mut n2 = 6_i64;
        let mut n3 = 6_i64;
        for _i in 1..n {
            let t2 = n3 * 2 + n2 * 3;
            let t3 = n3 * 2 + n2 * 2;
            n2 = t2 % MODULO;
            n3 = t3 % MODULO;
        }
        return ((n2 + n3) % MODULO) as i32;
    }
}

struct Solution;

fn main() {
    Solution::num_of_ways(12);
}
