//! 2023-01  剑指 Offer  

/// 2023-01-01  
/// 剑指 Offer 30. 包含min函数的栈  
/// <https://leetcode.cn/problems/bao-han-minhan-shu-de-zhan-lcof/>
pub struct Solution;
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
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
/// 2023-01-02  
/// 剑指 Offer 06. 从尾到头打印链表  
/// <https://leetcode.cn/problems/cong-wei-dao-tou-da-yin-lian-biao-lcof/>
impl Solution {
    pub fn reverse_print(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut ans = Vec::new();
        while let Some(node) = head {
            ans.push(node.val);
            head = node.next;
        }
        ans.reverse();
        ans
    }
    pub fn reverse_print2(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut head = head;
        let mut answer = Vec::new();
        while let Some(mut node) = head {
            head = node.next.take();
            answer.push(node.val);
        }
        answer.reverse();
        answer
    }
}
/// 2023-01-02  
/// Offer 24. 反转链表  
/// <https://leetcode.cn/problems/fan-zhuan-lian-biao-lcof/>
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut prev = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}
/// 2023-01-02  
/// 剑指 Offer 35. 复杂链表的复制  
/// <https://leetcode.cn/problems/fu-za-lian-biao-de-fu-zhi-lcof/>
impl Solution {
    /// 无 Rust 版本
    pub fn copy_random_list() {
        // TODO
        // class Solution {
        //     public:
        //         unordered_map<Node*, Node*> cachedNode;
        //         Node* copyRandomList(Node* head) {
        //             if (head == nullptr) {
        //                 return nullptr;
        //             }
        //             if (!cachedNode.count(head)) {
        //                 Node* headNew = new Node(head->val);
        //                 cachedNode[head] = headNew;
        //                 headNew->next = copyRandomList(head->next);
        //                 headNew->random = copyRandomList(head->random);
        //             }
        //             return cachedNode[head];
        //         }
        //     };
    }
}
#[test]
fn test_copy_random_list() {
    dbg!(Solution::copy_random_list());
    // assert_eq!(Solution::solve(),"output");
    // TODO
}
