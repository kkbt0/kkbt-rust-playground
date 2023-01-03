//! 2023-01  用于每日一题  
use std::collections::HashSet;

/// 通用 Solution
pub struct Solution;
/// 2023-01-01  
/// 2351. 第一个出现两次的字母  
/// <https://leetcode.cn/problems/first-letter-to-appear-twice/>
impl Solution {
    /// HashSet 去重 or dp
    pub fn repeated_character(s: String) -> char {
        let s: Vec<char> = s.chars().collect();
        let mut set = HashSet::new();
        for i in s.iter() {
            if set.contains(&i) {
                return *i;
            } else {
                set.insert(i);
            }
        }
        '0'
    }
}
#[test]
fn test_repeated_character() {
    assert_eq!(Solution::repeated_character("abccbaacz".to_string()), 'c');
    assert_eq!(Solution::repeated_character("abcdd".to_string()), 'd');
    // assert_eq!(Solution::solve(),"output");
    // TODO
}


/// 2023-01-02  
/// 1801. 积压订单中的订单总数  
/// <https://leetcode.cn/problems/number-of-orders-in-the-backlog/>
impl Solution {
    /// 优先队列模拟 Rust 版本  
    /// 已发题解 <https://leetcode.cn/problems/number-of-orders-in-the-backlog/solution/rust-guan-fang-jie-fa-by-kkbt-c3kn/>
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        const MOD: i32 = 1000000007;
        use std::collections::BinaryHeap; // price  amount
        let mut buy_orders = BinaryHeap::<(i32, i32)>::new(); //
        let mut sell_orders = BinaryHeap::<(i32, i32)>::new(); // Reverse

        for order in orders {
            let (price, mut amount, order_type) = (order[0], order[1], order[2]);
            if order_type == 0 {
                while amount > 0
                    && !sell_orders.is_empty()
                    && -sell_orders.peek().unwrap().0 <= price
                {
                    let mut sell_order = sell_orders.pop().unwrap();
                    let sell_amount = sell_order.1.min(amount);
                    amount -= sell_amount;
                    sell_order.1 -= sell_amount;
                    if sell_order.1 > 0 {
                        sell_orders.push(sell_order);
                    }
                }
                if amount > 0 {
                    buy_orders.push((price, amount));
                }
            } else {
                while amount > 0 && !buy_orders.is_empty() && buy_orders.peek().unwrap().0 >= price
                {
                    let mut buy_order = buy_orders.pop().unwrap();
                    let buy_amount = buy_order.1.min(amount);
                    amount -= buy_amount;
                    buy_order.1 -= buy_amount;
                    if buy_order.1 > 0 {
                        buy_orders.push(buy_order);
                    }
                }
                if amount > 0 {
                    sell_orders.push((-price, amount)); // for Rust BinaryHeap 只有大顶堆
                }
            }
        }
        let mut total = 0;
        for iter in sell_orders.iter() {
            total = (iter.1 + total) % MOD;
        }
        for iter in buy_orders.iter() {
            total = (iter.1 + total) % MOD;
        }
        return total;
    }
}
#[test]
fn test_get_number_of_backlog_orders() {
    assert_eq!(
        Solution::get_number_of_backlog_orders(vec![
            vec![10, 5, 0],
            vec![15, 2, 1],
            vec![25, 1, 1],
            vec![30, 4, 0]
        ]),
        6
    );
    assert_eq!(
        Solution::get_number_of_backlog_orders(vec![
            vec![7, 1000000000, 1],
            vec![15, 3, 0],
            vec![5, 999999995, 0],
            vec![5, 1, 1]
        ]),
        999999984
    );
    assert_eq!(
        Solution::get_number_of_backlog_orders(vec![
            vec![1, 29, 1],
            vec![22, 7, 1],
            vec![24, 1, 0],
            vec![25, 15, 1],
            vec![18, 8, 1],
            vec![8, 22, 0],
            vec![25, 15, 1],
            vec![30, 1, 1],
            vec![27, 30, 0]
        ]),
        22
    );
    assert_eq!(
        Solution::get_number_of_backlog_orders(vec![
            vec![23, 17, 1],
            vec![18, 27, 0],
            vec![21, 26, 1],
            vec![8, 17, 0],
            vec![13, 22, 1],
            vec![22, 21, 1],
            vec![2, 24, 1],
            vec![5, 7, 0]
        ]),
        69
    );
    // assert_eq!(Solution::solve(),"output");
    // TODO
}
