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

/* Make this program compile and run by removing
 * exactly two characters from the function below.
 */

fn foo() -> i32 {
    let p: &i32;
    
    {
        let x = 5;
        p = &x;
    }
    
    *p
}

fn main() {

    assert_eq!(foo(), 5);

    println!("\nAll tests passed!!\n");
   
    //submit::submit(Question(19), Score(2));
}
