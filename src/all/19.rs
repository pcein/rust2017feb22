/* These lines are required to facilitate
 * the score submission process. Don't worry
 * about them!
 */
#![allow(dead_code, unused_imports, unused_variables)]

mod submit;
mod myconfig;

extern crate hyper;

use submit::{Question, Score};

/*--------------------------------------------*/

fn is_prime(n: i32) -> bool {
    /* add code here */
}

fn main() {

    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(7), true);
    assert_eq!(is_prime(8), false);
    assert_eq!(is_prime(11), true);
    assert_eq!(is_prime(100003), true);


    println!("\nAll tests passed!!\n");
   
    //submit::submit(Question(20), Score(15));
}
