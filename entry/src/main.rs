use std::collections::BTreeSet;
fn main() {
    println!("Hello, world!");
    let mut set = BTreeSet::new();
    set.insert(3);
    set.insert(2);
    set.insert(1);
    println!("{:?}", set);
}
