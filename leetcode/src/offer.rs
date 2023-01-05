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

/// 2023-01-03  
/// 剑指 Offer 05. 替换空格  
/// <https://leetcode.cn/problems/ti-huan-kong-ge-lcof/>
impl Solution {
    /// Code_Description
    pub fn replace_space(s: String) -> String {
        s.replace(" ", "%20")
    }
}
#[test]
fn test_replace_space() {
    assert_eq!(
        Solution::replace_space("We are happy.".to_string()),
        "We%20are%20happy.".to_string()
    );
}

/// 2023-01-03  
/// 剑指 Offer 58 - II. 左旋转字符串  
/// <https://leetcode.cn/problems/zuo-xuan-zhuan-zi-fu-chuan-lcof/>
impl Solution {
    /// Code_Description
    pub fn reverse_left_words(s: String, n: i32) -> String {
        let mut s = Vec::from(s);
        for _ in 0..n {
            let char = s.remove(0);
            s.push(char);
        }
        String::from_utf8(s).unwrap()
    }
    // 字符串切片
    pub fn reverse_left_words2(s: String, n: i32) -> String {
        [&s[n as usize..], &s[..n as usize]].concat()
    }
}
#[test]
fn test_solve() {
    // dbg!(Solution::reverse_left_words());
    assert_eq!(
        Solution::reverse_left_words("abcdefg".to_string(), 2),
        "cdefgab".to_string()
    );
    // TODO
}
/// 2023-01-04  
/// 剑指 Offer 03. 数组中重复的数字  
/// <https://leetcode.cn/problems/shu-zu-zhong-zhong-fu-de-shu-zi-lcof/>
impl Solution {
    /// Code_Description
    pub fn find_repeat_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                return nums[i];
            }
        }
        0
    }
}
#[test]
fn test_find_repeat_number() {
    dbg!(Solution::find_repeat_number(vec![2, 3, 1, 0, 2, 5, 3]));
    // assert_eq!(Solution::solve(),"output");
    // TODO
}

/// 2023-01-04  
/// 剑指 Offer 53 - I. 在排序数组中查找数字 I  
/// <https://leetcode.cn/problems/zai-pai-xu-shu-zu-zhong-cha-zhao-shu-zi-lcof/>
impl Solution {
    /// Code_Description
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        nums.iter().filter(|x| x == &&target).count() as i32
    }
    pub fn search2(nums: Vec<i32>, target: i32) -> i32 {
        let find = nums.binary_search(&target);
        println!("{:?}", find);
        if find.is_ok() {
            let mut sum = 0;
            for i in find.unwrap()..nums.len() {
                if nums[i] == target {
                    sum += 1;
                } else {
                    break;
                }
            }
            for i in (0..find.unwrap()).rev() {
                if nums[i] == target {
                    sum += 1;
                } else {
                    break;
                }
            }
            return sum;
        }
        0
    }
}
#[test]
fn test_search() {
    assert_eq!(Solution::search2(vec![5, 7, 7, 8, 8, 10], 8), 2);
    assert_eq!(Solution::search2(vec![5, 7, 7, 8, 8, 10], 1), 0);
    assert_eq!(Solution::search2(vec![1], 1), 1);
    assert_eq!(Solution::search2(vec![2, 2], 2), 2);
}

/// 2023-01-04  
/// 剑指 Offer 53 - II. 0～n-1中缺失的数字  
/// <https://leetcode.cn/problems/que-shi-de-shu-zi-lcof/>
impl Solution {
    /// 正常是用二分的 偷个懒
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            if nums[i] != i as i32 {
                return i as i32;
            }
        }
        *nums.last().unwrap() + 1
    }
}
#[test]
fn test_missing_number() {
    assert_eq!(Solution::missing_number(vec![0, 1, 3]), 2);
    assert_eq!(Solution::missing_number(vec![0]), 1);
}
/// 2023-01-05  
/// 剑指 Offer 04. 二维数组中的查找  
/// <https://leetcode.cn/problems/er-wei-shu-zu-zhong-de-cha-zhao-lcof/>
impl Solution {
    /// 不如暴力搜索
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        if m == 0 || matrix[0].len() == 0 {
            return false;
        }
        let n = matrix[0].len();
        for i in 0..matrix.len() {
            if matrix[i][0] <= target && matrix[i][n - 1] >= target {
                if matrix[i].binary_search(&target).is_ok() {
                    return true;
                }
            }
        }
        false
    }
}
#[test]
fn test_find_number_in2_d_array() {
    // dbg!(Solution::solve());
    assert_eq!(
        Solution::find_number_in2_d_array(
            vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30]
            ],
            5
        ),
        true
    );
    // TODO
}

/// 2023-01-05  
/// 剑指 Offer 11. 旋转数组的最小数字  
/// <https://leetcode.cn/problems/xuan-zhuan-shu-zu-de-zui-xiao-shu-zi-lcof/>
impl Solution {
    /// Code_Description
    pub fn min_array(numbers: Vec<i32>) -> i32 {
        *numbers.iter().min().unwrap()
    }
}
#[test]
fn test_min_array() {
    dbg!(Solution::min_array(vec![3, 4, 5, 1, 2]), 1);
}

/// 2023-01-05  
/// 剑指 Offer 50. 第一个只出现一次的字符
/// <https://leetcode.cn/problems/di-yi-ge-zhi-chu-xian-yi-ci-de-zi-fu-lcof/>
impl Solution {
    /// Code_Description
    pub fn first_uniq_char(s: String) -> char {
        let mut arr = [0; 26];
        for iter in s.bytes() {
            arr[(iter - b'a') as usize] += 1;
        }
        for iter in s.bytes()  {
            if arr[(iter - b'a') as usize]  == 1 {
                return iter as char;
            }
        }
        ' '
    }
}
#[test]
fn test_first_uniq_char() {
    assert_eq!(Solution::first_uniq_char("abaccdeff".to_string()), 'b');
    assert_eq!(Solution::first_uniq_char("".to_string()), ' ');
    // assert_eq!(Solution::solve(),"output");
    // TODO
}
