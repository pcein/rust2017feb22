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

// translate a single char
fn translate(c: char) -> char {
    (b'a' + (((c as u8 - b'a') + 13) % 26)) as char
}

//rot13 for lower case strings
fn rot13(s: &str) -> String {
    
    let r:String = /* Add code here */

    r
}


fn main() {

    assert_eq!(rot13("abc"), "nop");
    assert_eq!(rot13("mnop"), "zabc");

    println!("\nAll tests passed!!\n");
   
    //submit::submit(Question(11), Score(10));
}
