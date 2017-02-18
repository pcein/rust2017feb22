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

// Factorial of non-zero integers without using a loop

fn factorial(n: i32) -> i32 {
    let r:i32 = /* add a line here */

    r
} 

fn main() {

    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(3), 6);
    assert_eq!(factorial(4), 24);
 
    println!("\nAll tests passed!!\n");   
    
    //submit::submit(Question(6), Score(5));
}
