struct MyStack {
    arr: [i32; 100],
    pos: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyStack {
            arr: [0; 100],
            pos: 0,
        }
    }
    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.arr[self.pos] = x;
        self.pos += 1;
    }
    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        self.pos -= 1;
        self.arr[self.pos]
    }
    /** Get the top element. */
    fn top(&self) -> i32 {
        self.arr[self.pos - 1]
    }
    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.pos == 0
    }
}

fn main() {
    let mut obj = MyStack::new();
    obj.push(10);
    obj.top();
    obj.pop();
    obj.empty();
}
