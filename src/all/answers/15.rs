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

#[derive(Debug, PartialEq)]
struct Second(i32); 

#[derive(Debug, PartialEq)]
struct MilliSecond(i32);

fn seconds2milliseconds(s: Second) -> MilliSecond {

    match s {
        Second(x) => MilliSecond(x * 1000),
    }

}

fn main() {

    assert_eq!(seconds2milliseconds(Second(1)), MilliSecond(1000));
    assert_eq!(seconds2milliseconds(Second(2)), MilliSecond(2000));
    assert_eq!(seconds2milliseconds(Second(0)), MilliSecond(0));


    println!("\nAll tests passed!!\n");
   
    //submit::submit(Question(15), Score(10));
}
