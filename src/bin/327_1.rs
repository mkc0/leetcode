use std::cell::RefCell;

struct Node {
    level: i8,
    count: i32,
    child: [Option<Box<RefCell<Node>>>; 2],
}

impl Node {
    fn new(level: i8) -> Self {
        Node {
            level: level,
            count: 0,
            child: [None, None],
        }
    }

    fn child_sum(&self, ch: usize) -> i32 {
        if let Some(cell) = &self.child[ch] {
            return cell.borrow().count;
        }
        0
    }

    fn insert(&mut self, val: u64) {
        if self.level < 0 {
            self.count += 1;
            return;
        }
        let base = 1_u64 << self.level;
        let child;
        let after_val;
        if (base & val) == base {
            after_val = val ^ base;
            child = &mut self.child[1];
        } else {
            after_val = val;
            child = &mut self.child[0];
        }
        if let Some(cell) = child {
            cell.borrow_mut().insert(after_val);
        } else {
            let cell = Box::new(RefCell::new(Node::new(self.level - 1)));
            cell.borrow_mut().insert(after_val);
            *child = Some(cell);
        }
        self.count = self.child_sum(0) + self.child_sum(1);
    }

    fn delete(&mut self, val: u64) {
        if self.level < 0 {
            self.count -= 1;
            return;
        }
        let base = 1_u64 << self.level;
        let child;
        let after_val;
        if (base & val) == base {
            after_val = val ^ base;
            child = &mut self.child[1];
        } else {
            after_val = val;
            child = &mut self.child[0];
        }
        if let Some(cell) = child {
            cell.borrow_mut().delete(after_val);
        } else {
            panic!("not go here");
        }
        self.count = self.child_sum(0) + self.child_sum(1);
    }

    fn count_bound(&self, bound: u64) -> i32 {
        if self.level < 0 {
            return self.count;
        }
        let base = 1_u64 << self.level;
        let child;
        let after_bound;
        let mut count = 0;
        if (base & bound) == base {
            after_bound = bound ^ base;
            child = &self.child[1];
            count += self.child_sum(0);
        } else {
            after_bound = bound;
            child = &self.child[0];
        }
        if let Some(cell) = child {
            count += cell.borrow().count_bound(after_bound);
        }
        count
    }
}

impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let top_level = 50_i8;
        let bias_to_positive = 100_001_i64 * (1i64 << 31);
        let mut sum = 0_i64;
        let sums: Vec<i64> = nums
            .iter()
            .map(|num| {
                sum += *num as i64;
                sum
            })
            .collect();
        if lower > upper {
            return 0;
        }
        let mut count = 0;
        let mut root = Node::new(top_level);
        for s in sums.iter() {
            let bias_s = (bias_to_positive + *s) as u64;
            root.insert(bias_s);
        }
        let mut sum = 0_i64;
        for num in nums.iter() {
            let lower = (lower as i64 + sum + bias_to_positive) as u64;
            let upper = (upper as i64 + sum + bias_to_positive) as u64;
            count += root.count_bound(upper) - root.count_bound(lower - 1);
            sum += *num as i64;
            root.delete((sum + bias_to_positive) as u64);
        }
        count
    }
}

// fn print_type_name<T>(_ : &T) {
//     println!("{}", std::any::type_name::<T>());
// }

struct Solution;

fn main() {
    let a = vec![2147483647, -2147483648, -1, 0];
    println!("{}", Solution::count_range_sum(a, -1, 0));
}
