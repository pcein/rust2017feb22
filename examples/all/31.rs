
use std::collections::HashMap;

/* Explicit type declaration for a hashmap */

fn main() {
    
    let mut h:HashMap<&str, i32> = HashMap::new();

    h.insert("orange", 100);
    h.insert("apple", 200);
    h.insert("mango", 250);

    println!("price/kilo of orange = {}", h.get("orange").unwrap());
    println!("price/kilo of mango = {}", h.get("mango").unwrap());

}
