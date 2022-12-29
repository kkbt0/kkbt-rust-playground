# Rust Cheat

[https://cheats.rs/](https://cheats.rs/)

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
