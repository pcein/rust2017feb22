/* These lines are required to facilitate
 * the score submission process. Don't worry
 * about them!
 */
 
mod submit;
mod myconfig;

extern crate hyper;

use submit::{Question, Score};

/*--------------------------------------------*/


fn factorial(mut n: i32) -> i32 {
    
    let  f = 1; 

    while n > 0 {
        /* add a line here */

        n = n - 1;
    }
    f
}


fn main() {

    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(3), 6);
    assert_eq!(factorial(5), 120);

  //  submit::submit(Question(3), Score(5));
}
