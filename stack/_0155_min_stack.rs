// https://leetcode.com/problems/min-stack
//
// Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
//
// Implement the `MinStack` class:
//
// *   `MinStack()` initializes the stack object.
// *   `void push(int val)` pushes the element `val` onto the stack.
// *   `void pop()` removes the element on the top of the stack.
// *   `int top()` gets the top element of the stack.
// *   `int getMin()` retrieves the minimum element in the stack.
//
// **Example 1:**
//
// ```
// **Input**
// ["MinStack","push","push","push","getMin","pop","top","getMin"]
// [[],[-2],[0],[-3],[],[],[],[]]
//
// **Output**
// [null,null,null,null,-3,null,0,-2]
//
// **Explanation**
// MinStack minStack = new MinStack();
// minStack.push(-2);
// minStack.push(0);
// minStack.push(-3);
// minStack.getMin(); // return -3
// minStack.pop();
// minStack.top();    // return 0
// minStack.getMin(); // return -2
// ```
//
// **Constraints:**
//
// *   `-2<sup>31</sup> <= val <= 2<sup>31</sup> - 1`
// *   Methods `pop`, `top` and `getMin` operations will always be called on **non-empty** stacks.
// *   At most `3 * 10<sup>4</sup>` calls will be made to `push`, `pop`, `top`, and `getMin`.

struct MinStack {
    data: Vec<i32>,
    min: i32,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            data: Vec::new(),
            min: std::i32::MAX,
        }
    }

    fn push(&mut self, val: i32) {
        self.data.push(val);
        if self.min > val {
            self.min = val;
        }
    }

    fn pop(&mut self) {
        let pop = self.data.pop().unwrap();
        if pop == self.min {
            if !(self.data.is_empty()) {
                self.min = *self.data.iter().min().unwrap();
            } else {
                self.min = std::i32::MAX;
            }
        }
    }

    fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        return self.min;
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[test]
pub fn t1() {
    let mut obj = MinStack::new();
    obj.push(-2);
    obj.push(0);
    obj.push(-3);
    assert_eq!(obj.get_min(), -3);
    obj.pop();
    assert_eq!(obj.top(), 0);
    assert_eq!(obj.get_min(), -2);
}
