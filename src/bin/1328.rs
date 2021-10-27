impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let mut cc = palindrome.clone();
        let bs;
        unsafe {
            bs = cc.as_mut_vec();
        }
        if bs.len() == 1 {
            return String::from("");
        }
        let n = bs.len();
        let bads: Vec<usize> = (0..n).filter(|x| bs[*x] != bs[n - 1 - *x]).collect();
        if bads.len() > 2 {
            // All can change
        } else if bads.len() == 1 {
            // All but bad can not be same
        } else {
            // All but center
            for i in 0..n {
                if n % 2 != 1 || i * 2 + 1 != n {
                    if bs[i] != b'a' {
                        bs[i] = b'a';
                        return String::from_utf8(bs.to_vec()).unwrap();
                    }
                }
            }
            bs[n-1] = b'b';
            return String::from_utf8(bs.to_vec()).unwrap();
        }
        String::from("")
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::break_palindrome(String::from("aba")));
}
