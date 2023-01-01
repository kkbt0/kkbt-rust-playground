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
/// 2022-12-30  
/// 21. 合并两个有序链表  
/// <https://leetcode.cn/problems/merge-two-sorted-lists/>  
impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head_node = ListNode::new(0);
        let mut node = &mut head_node;
        while let (Some(l1), Some(l2)) = (list1.as_ref(), list2.as_ref()) {
            if l1.val < l2.val {
                node.next = list1; // head(no meaning).next -> list1
                node = node.next.as_mut().unwrap(); // node = list1.next 指向 list1 第一项
                list1 = node.next.take(); // node 指向 list1 第二项
            } else {
                node.next = list2;
                node = node.next.as_mut().unwrap();
                list2 = node.next.take();
            }
        }
        node.next = if list1.is_some() { list1 } else { list2 };
        head_node.next
    }
    /// <https://leetcode.cn/problems/merge-two-sorted-lists/solution/rust-by-tryfor_-23/>
    pub fn merge_two_lists3(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l3 = ListNode::new(0);
        let mut ptr3 = &mut l3;
        while let (Some(n1), Some(n2)) = (l1.as_ref(), l2.as_ref()) {
            if n1.val < n2.val {
                //将较小链表连接到新链表尾节点，所有权移动
                ptr3.next = l1;
                //将l3尾节点指向它的后继节点
                ptr3 = ptr3.next.as_mut().unwrap();
                //将链表从尾节点取下来，将所有权返给较小的链表
                l1 = ptr3.next.take();
            } else {
                //同上面的逻辑
                ptr3.next = l2;
                ptr3 = ptr3.next.as_mut().unwrap();
                l2 = ptr3.next.take();
            }
        }
        //剩余部分
        ptr3.next = if l1.is_some() { l1 } else { l2 };
        l3.next
    }
    /// 递归方法  <https://leetcode.cn/problems/merge-two-sorted-lists/solution/rust-di-gui-he-bing-shuang-lian-biao-by-sniper_mar/>
    pub fn merge_two_lists2(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => {
                if l.val <= r.val {
                    l.next = Self::merge_two_lists2(l.next, Some(r));
                    Some(l)
                } else {
                    r.next = Self::merge_two_lists2(Some(l), r.next);
                    Some(r)
                }
            }
        }
    }
}

#[test]
fn test_merge_two_lists() {
    // 输入：l1 = [1,2,4], l2 = [1,3,4]
    // 输出：[1,1,2,3,4,4]
    let l1_3 = ListNode::new(4);
    let mut l1_2 = ListNode::new(2);
    let mut l1_1 = ListNode::new(1);
    l1_2.next = Some(Box::new(l1_3));
    l1_1.next = Some(Box::new(l1_2));

    let l2_3 = ListNode::new(4);
    let mut l2_2 = ListNode::new(3);
    let mut l2_1 = ListNode::new(1);
    l2_2.next = Some(Box::new(l2_3));
    l2_1.next = Some(Box::new(l2_2));

    let l3_6 = ListNode::new(4);
    let mut l3_5 = ListNode::new(4);
    let mut l3_4 = ListNode::new(3);
    let mut l3_3 = ListNode::new(2);
    let mut l3_2 = ListNode::new(1);
    let mut l3_1 = ListNode::new(1);
    l3_5.next = Some(Box::new(l3_6));
    l3_4.next = Some(Box::new(l3_5));
    l3_3.next = Some(Box::new(l3_4));
    l3_2.next = Some(Box::new(l3_3));
    l3_1.next = Some(Box::new(l3_2));

    let left = Solution::merge_two_lists2(Some(Box::new(l1_1)), Some(Box::new(l2_1)));
    let right = Some(Box::new(l3_1));

    assert_eq!(left, right);
}
/// 2022-12-30  
/// 35. 搜索插入位置  
/// <https://leetcode.cn/problems/search-insert-position/>
impl Solution {
    /// 正常是二分查找。不过 Rust 有库
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }
}
#[test]
fn test_search_insert() {
    // dbg!(Solution::search_insert(vec![1,3,5,6],5));
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    // TODO
}
use std::cmp::max;
use std::collections::HashSet;
use std::vec;
/// 2022-12-30  
/// 3. 无重复字符的最长子串  
/// <https://leetcode.cn/problems/longest-substring-without-repeating-characters/?favorite=2cktkvj>
/// - [ ] **未解决**
impl Solution {
    /// 滑动窗口
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        if s.len() == 1 {
            return 1;
        }
        let mut max_len = 0;
        let mut left = 0;
        let mut set = HashSet::new();
        for i in 0..s.len() {
            if set.contains(&s[i]) {
                max_len = max(max_len, set.len());
                loop {
                    // pwwkew
                    if set.contains(&s[left]) {
                        println!("{:?}", set);
                        left += 1;
                        break;
                    } else {
                        let x = set.remove(&s[left]);
                        println!("{}", x);
                        left += 1;
                    }
                }
            } else {
                set.insert(s[i]); // anvi aj
            }
        }
        max_len = max(max_len, set.len());
        max_len as i32
    }
}
#[rustfmt::skip]
#[test]
fn test_length_of_longest_substring() {
    // assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()),3); // 3
    // assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()),1); // 1
    // assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()),3); // 3
    // assert_eq!(Solution::length_of_longest_substring(" ".to_string()),1); // 1
    // assert_eq!(Solution::length_of_longest_substring("".to_string()),0); // 
    // assert_eq!(Solution::length_of_longest_substring("au".to_string()),2); // 
    // assert_eq!(Solution::length_of_longest_substring("anviaj".to_string()),5);
    // assert_eq!(Solution::solve(),"output");
    // TODO
}
/// 2023-01-01  
/// 4. 寻找两个正序数组的中位数  
/// <https://leetcode.cn/problems/median-of-two-sorted-arrays/>
impl Solution {
    /// 库 chain sort 浪费了两个正序
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut union = nums1.iter().chain(&nums2).collect::<Vec<_>>();
        union.sort();
        let len = union.len();
        if len % 2 == 0 {
            return (union[len / 2 - 1] + union[len / 2]) as f64 / 2.0;
        } else {
            return *union[len / 2] as f64;
        }
    }
}
#[test]
fn test() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
        2.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        2.5
    );
}
