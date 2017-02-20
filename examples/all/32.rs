
use std::collections::HashMap;


fn main() {
    
    let mut h = HashMap::new();

    h.insert("abc", 10);

    *h.entry("abc").or_insert(0) += 1;

    println!("value associated with abc = {}", h.get("abc").unwrap());


    *h.entry("def").or_insert(0) += 1;

    println!("value associated with def = {}", h.get("def").unwrap());

}
