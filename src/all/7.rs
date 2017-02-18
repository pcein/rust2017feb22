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

// sum all even numbers from 1 to n-1

fn sum_even_numbers(n: i32) -> i32 {
    let r = /* add a line here */

    r
} 

fn main() {

    assert_eq!(sum_even_numbers(3), 2);
    assert_eq!(sum_even_numbers(6), 6);
    assert_eq!(sum_even_numbers(10), 20);
    
    //submit::submit(Question(7), Score(5));
}
