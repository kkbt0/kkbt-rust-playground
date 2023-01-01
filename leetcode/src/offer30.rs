/// 2023-01-01  
/// 剑指 Offer 30. 包含min函数的栈  
/// <https://leetcode.cn/problems/bao-han-minhan-shu-de-zhan-lcof/>
#[allow(dead_code)]
struct MinStack {
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
