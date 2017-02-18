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


fn main() {

    assert_eq!(sum_even_numbers(3), 2);
    assert_eq!(sum_even_numbers(6), 6);
    assert_eq!(sum_even_numbers(10), 20);
    
    //submit::submit(Question(8), Score(10));
}
