impl Solution {
    fn dfs(
        x: usize,
        cnter: &mut u32,
        edgs: &Vec<Vec<usize>>,
        dfn: &mut Vec<u32>,
        ins: &mut Vec<bool>
    ) -> bool {
        *cnter += 1;
        dfn[x] = *cnter;
        ins[x] = true;
        for y in edgs[x].iter() {
            let y = *y;
            if dfn[y] == 0 {
                if !Solution::dfs(y, cnter, edgs, dfn, ins) {
                    return false;
                }
                if dfn[x] > dfn[y] {
                    return false;
                }
            } else if ins[y] {
                if dfn[x] > dfn[y] {
                    return false;
                }
            }
        }
        ins[x] = false;
        true
    }

    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut edgs = vec![Vec::<usize>::new(); num_courses];
        let mut dfn = vec![0u32; num_courses];
        let mut ins = vec![false; num_courses];
        for p in prerequisites.iter() {
            edgs[p[0] as usize].push(p[1] as usize);
            if p[0] == p[1] {
                return false;
            }
        }
        let mut cnter = 0u32;
        for i in 0..num_courses {
            if dfn[i] == 0 {
                if !Solution::dfs(i, &mut cnter, &edgs, &mut dfn, &mut ins) {
                    return false;
                }
            }
        }
        true
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::can_finish(2, vec![vec![0, 1], vec![1, 0]]));
    println!(
        "{}",
        Solution::can_finish(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]])
    );
    println!(
        "{}",
        Solution::can_finish(3, vec![vec![0, 1], vec![0, 2], vec![1, 2]])
    );
}
