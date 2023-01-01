//! 2023-01  剑指 Offer  

/// 2023-01-01  
/// 剑指 Offer 30. 包含min函数的栈  
/// <https://leetcode.cn/problems/bao-han-minhan-shu-de-zhan-lcof/>
#[allow(dead_code)]
pub struct MinStack {
    val: Vec<i32>,
}
#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        MinStack { val: Vec::new() }
    }
    fn push(&mut self, x: i32) {
        self.val.push(x);
    }
    fn pop(&mut self) {
        self.val.pop();
    }
    fn top(&self) -> i32 {
        self.val[self.val.len() - 1]
    }
    fn min(&self) -> i32 {
        *self.val.iter().min().unwrap()
    }
}
/// 2023-01-01  
/// 剑指 Offer 09. 用两个栈实现队列  
/// <https://leetcode.cn/problems/yong-liang-ge-zhan-shi-xian-dui-lie-lcof/>
#[allow(dead_code)]
pub struct CQueue {
    vec: Vec<i32>,
}
#[allow(dead_code)]
impl CQueue {
    fn new() -> Self {
        Self { vec: Vec::new() }
    }
    fn append_tail(&mut self, value: i32) {
        self.vec.push(value)
    }
    fn delete_head(&mut self) -> i32 {
        if self.vec.is_empty() {
            return -1;
        }
        self.vec.remove(0)
    }
}
