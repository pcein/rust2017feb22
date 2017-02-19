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

fn first<T1, T2, T3>(x: (T1, T2, T3)) -> T1 {
    x.0
}

fn main() {

    assert_eq!(first((1,2,3)), 1);
    assert_eq!(first(("hello", 10, 20)), "hello");
    assert_eq!(first(('A', "world", 1.2)), 'A');
    assert_eq!(first((vec![1,2], 10, "hello")), vec![1,2]);

    println!("\nAll tests passed!!\n");
   
    //submit::submit(Question(17), Score(10));
}
