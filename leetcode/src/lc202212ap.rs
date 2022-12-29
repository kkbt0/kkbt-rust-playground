//! 用于 leetcode 所有问题 非每日一题
#[allow(dead_code)]
pub struct Solution;
/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
/// 2022-12-29  
/// 2. 两数相加  
/// <https://leetcode.cn/problems/add-two-numbers/>
impl Solution {
    /// 链表 不太擅长使用各类函数
    #[rustfmt::skip]
    pub fn add_two_numbers( l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let (mut l1, mut l2) = (l1,l2);
            let mut vec = Vec::new();
            let mut jw = false;
            loop {
                // Box::new(ListNode::new(0))
                let v1 = l1.as_ref().unwrap_or(&Box::new(ListNode::new(0))).val;
                let v2 = l2.as_ref().unwrap_or(&Box::new(ListNode::new(0))).val;
                let num = v1 + v2 + if jw { 1 } else { 0 };
                jw = false;
                if (num) >= 10 {
                    jw = true;
                }
                vec.push((num) % 10);
                l1 = l1.as_ref().unwrap_or(&Box::new(ListNode::new(0))).next.clone();
                l2 = l2.as_ref().unwrap_or(&Box::new(ListNode::new(0))).next.clone();
                if let (None, None) = (&l1, &l2) {
                    if jw { vec.push(1);}
                    break;
                }
            }
            let mut head_node = ListNode::new(vec[0]);
            let mut node = &mut head_node;
            for i in 1..vec.len() {
                node.next = Some(Box::new(ListNode::new(vec[i])));
                node = node.next.as_mut().unwrap();
            }
            Some(Box::new(head_node))
        }
}
#[test]
fn test_add_two_numbers() {
    // 输入：l1 = [2,4,3], l2 = [5,6,4]
    // 输出：[7,0,8]
    // 解释：342 + 465 = 807.
    let l1_3 = ListNode::new(3);
    let mut l1_2 = ListNode::new(4);
    let mut l1_1 = ListNode::new(2);
    l1_2.next = Some(Box::new(l1_3));
    l1_1.next = Some(Box::new(l1_2));

    let l2_3 = ListNode::new(4);
    let mut l2_2 = ListNode::new(6);
    let mut l2_1 = ListNode::new(5);
    l2_2.next = Some(Box::new(l2_3));
    l2_1.next = Some(Box::new(l2_2));

    let l3_3 = ListNode::new(8);
    let mut l3_2 = ListNode::new(0);
    let mut l3_1 = ListNode::new(7);
    l3_2.next = Some(Box::new(l3_3));
    l3_1.next = Some(Box::new(l3_2));

    let left = Solution::add_two_numbers(Some(Box::new(l1_1)), Some(Box::new(l2_1)));
    let right = Some(Box::new(l3_1));

    assert_eq!(left, right);
}

/// 2022-12-29  
/// 70. 爬楼梯  
/// <https://leetcode.cn/problems/climbing-stairs/>
impl Solution {
    /// dp
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp: [i32; 45] = [0; 45];
        dp[0] = 1;
        dp[1] = 2;
        for i in 2..n as usize {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n as usize - 1]
    }
}
#[test]
fn test_climb_stairs() {
    assert_eq!(Solution::climb_stairs(1), 1);
    assert_eq!(Solution::climb_stairs(2), 2);
    assert_eq!(Solution::climb_stairs(3), 3);
}
/// 2022-12-29  
/// 13. 罗马数字转整数
/// <https://leetcode.cn/problems/roman-to-integer/>
impl Solution {
    /// 两次模式匹配
    pub fn roman_to_int(s: String) -> i32 {
        let mut vec = Vec::new();
        let byte = s.as_bytes();
        for bchar in byte {
            match bchar {
                b'I' => vec.push(1),
                b'V' => vec.push(5),
                b'X' => vec.push(10),
                b'L' => vec.push(50),
                b'C' => vec.push(100),
                b'D' => vec.push(500),
                b'M' => vec.push(1000),
                _ => {}
            }
        }
        let sub = byte.windows(2).fold(0, |mut acc, x| {
            match (x[0], x[1]) {
                (b'I', b'V') => acc -= 2,
                (b'I', b'X') => acc -= 2,
                (b'X', b'L') => acc -= 20,
                (b'X', b'C') => acc -= 20,
                (b'C', b'D') => acc -= 200,
                (b'C', b'M') => acc -= 200,
                _ => {}
            }
            acc
        });
        vec.iter().sum::<i32>() + sub
    }
    /// 模式匹配  
    /// <https://leetcode.cn/problems/roman-to-integer/solution/mo-shi-pi-pei-fold-zhi-jie-guo-by-howyea-fn9y/>
    pub fn roman_to_int2(s: String) -> i32 {
        s.chars()
            .fold((0, ' '), |res, ch| match (res.1, ch) {
                ('I', 'V') => (res.0 + 3, 'V'),
                ('I', 'X') => (res.0 + 8, 'X'),
                ('X', 'L') => (res.0 + 30, 'L'),
                ('X', 'C') => (res.0 + 80, 'C'),
                ('C', 'D') => (res.0 + 300, 'D'),
                ('C', 'M') => (res.0 + 800, 'M'),
                (_, 'I') => (res.0 + 1, 'I'),
                (_, 'V') => (res.0 + 5, 'V'),
                (_, 'X') => (res.0 + 10, 'X'),
                (_, 'L') => (res.0 + 50, 'L'),
                (_, 'C') => (res.0 + 100, 'C'),
                (_, 'D') => (res.0 + 500, 'D'),
                (_, 'M') => (res.0 + 1000, 'M'),
                (_, _) => unreachable!(),
            })
            .0
    }
}
#[test]
fn test_roman_to_int() {
    // dbg!(Solution::roman_to_int("III".to_string()));
    // dbg!(Solution::roman_to_int("IV".to_string()));
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}
