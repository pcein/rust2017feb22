
use std::collections::HashSet;

fn main() {
    let mut h = HashSet::new();
    
    h.insert("apple");
    h.insert("orange");
    h.insert("banana");
    
    println!("contains banana: {}", h.contains("banana"));
    println!("contains mango: {}", h.contains("mango"));

}
    
