// 用于 leetcode all problems
#[allow(dead_code)]
mod lc202212ap {
    struct Solution;

    // Definition for singly-linked list.
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
    // 2022-12-29
    // 2. 两数相加
    // https://leetcode.cn/problems/add-two-numbers/
    impl Solution {
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
}
