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

fn sqr(x: i32) -> i32 {
    x * x 
}

fn main() {

    assert_eq!(sqr(10), 100);
    assert_eq!(sqr(-1), 1);
    assert_eq!(sqr(0), 0);
 
    println!("All tests passed!!");  
    // submit::submit(Question(1), Score(0));
}
