# Rust Cheat

[https://cheats.rs/](https://cheats.rs/)

index.html 
```html
<meta http-equiv='refresh' content='0; url=/leetcode'>
```

## Rust 常用处理

```rust
fn main() {
    println!("Hello, world!");
    let str = String::from("cba");
    str.chars().enumerate().for_each(|(i, c)| {
        println!("index: {}, char: {}", i, c);
    });
}
// index: 0, char: c
// index: 1, char: b
// index: 2, char: a
```

大顶堆/ 优先队列

```rust
use std::collections::BinaryHeap;
let mut demo = BinaryHeap::<(i32, i32)>::new(); // 用小顶堆 直接 push((-1,2)); 之后用数据再 *(-1) 即可
demo.push((1, 2));
```