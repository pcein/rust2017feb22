/* These lines are required to facilitate
 * the score submission process. Don't worry
 * about them!
 */
#![allow(dead_code, unused_imports)]

mod submit;
mod myconfig;

extern crate hyper;

use submit::{Question, Score};

/*--------------------------------------------*/
use std::collections::HashMap;

fn word_count(s: &str) -> HashMap<&str, i32> {

    let mut h = HashMap::new();

    for word in s.split_whitespace() {
        *h.entry(word).or_insert(0) += 1;
    }

    h

}


fn main() {

    let h = word_count("abc def abc ijk abc def");

    assert_eq!(*h.get("abc").unwrap(), 3);
    assert_eq!(*h.get("def").unwrap(), 2);
    assert_eq!(*h.get("ijk").unwrap(), 1);

    println!("\nAll tests passed!!\n");
   
    //submit::submit(Question(12), Score(10));
}
