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

/* Make this program compile and run by adding a
 * single "{" and a single "}" to the function below.
 * No other change is allowed.
 */

fn foo() -> i32 {
    let mut v = vec![6,7,8];
    
    let p1 = &mut v;

    let p2 = &v;

    p2[0]
}

fn main() {

    assert_eq!(foo(), 6);

    println!("\nAll tests passed!!\n");
   
    //submit::submit(Question(18), Score(2));
}
