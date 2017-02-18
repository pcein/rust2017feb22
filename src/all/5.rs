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

// sum of all integers from a to b, b inclusive

fn sum_series(a: i32, b: i32) -> i32 {
    let sum = 0;
    for x in a .. b+1 {
        /* add a line here */
    }
    sum
} 

fn main() {

    assert_eq!(sum_series(1,1), 1);
    assert_eq!(sum_series(1,2), 3);
    assert_eq!(sum_series(1,5), 15);
    
    //submit::submit(Question(5), Score(5));
}
