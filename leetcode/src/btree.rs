// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        enum Visited {
            Yes,
            No,
        }
        let mut stack: Vec<(Option<Rc<RefCell<TreeNode>>>, Visited)> = vec![];
        let mut res = vec![];
        stack.push((root, Visited::No));
        loop {
            if stack.len() == 0 {
                break;
            }
            let st = stack.pop().unwrap();
            if let Some(node) = st.0.clone() {
                if let Visited::No = st.1 {
                    // 此处从下到上是中序遍历的顺序
                    // 前序遍历把中间那条放最后
                    // 后续遍历把中间那条放最前
                    stack.push((node.borrow().right.clone(), Visited::No));
                    stack.push((st.0.clone(), Visited::Yes));
                    stack.push((node.borrow().left.clone(), Visited::No));
                } else {
                    res.push(node.borrow().val);
                }
            }
        }
        res
    }
    /// <https://leetcode.cn/problems/binary-tree-inorder-traversal/solution/94-er-cha-shu-de-zhong-xu-bian-li-by-tab-a3ff/>
    pub fn inorder_traversal2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];
        let mut root = root;
        while !stack.is_empty() || root.is_some() {
            while let Some(node) = root {
                root = node.borrow_mut().left.take();
                stack.push(node);
            }
            if let Some(node) = stack.pop() {
                ans.push(node.borrow().val);
                root = node.borrow_mut().right.take();
            }
        }
        ans
    }
}
