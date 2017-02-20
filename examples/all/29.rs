
use std::collections::HashSet;

fn main() {
    let mut h1 = HashSet::new();
    let mut h2 = HashSet::new();

    h1.insert("orange");    

    h2.insert("apple");
    h2.insert("orange");
    h2.insert("banana");

    

    println!("h1 is subset of h2: {}", h1.is_subset(&h2));
    println!("h2 is subset of h1: {}", h2.is_subset(&h1));

}
    
