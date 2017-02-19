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

#[derive(Debug)]
struct Complex {
    re: i32,
    im: i32,
}

impl Complex {
    fn add(&self, other: Complex) -> Complex {

        /* add code here */
    }
}


fn main() {

    let a = Complex { re: 1, im: 2};
    let b = Complex { re: 10, im: 11};
    let c = a.add(b);

    assert_eq!(c.re, 11);
    assert_eq!(c.im, 13);

    println!("\nAll tests passed!!\n");
   
    //submit::submit(Question(16), Score(10));
}
