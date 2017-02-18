/* These lines are required to facilitate
 * the score submission process. Don't worry
 * about them!
 */
 
mod submit;
mod myconfig;

extern crate hyper;

use submit::{Question, Score};

/*--------------------------------------------*/

fn sqr(x: i32) -> i32 {
    x * x
}

fn is_pythagorean_triple() ->  {
    
}

fn main() {

    assert_eq!(is_pythagorean_triple(3, 4, 5), true);
    assert_eq!(is_pythagorean_triple(1, 2, 3), false);
    assert_eq!(is_pythagorean_triple(7, 8, 9), false);
 
    println!("\nAll tests passed!!\n");
   // submit::submit(Question(2), Score(5));
}
