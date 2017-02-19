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

enum Symbols {
    Num(i32),
    Add,
    Mul,
}

use Symbols::*;

fn eval(expr: Vec<Symbols>) -> i32 {

    let mut v = vec![];

    for symbol in expr {
        match symbol {
            Num(val) => v.push(val),
            
            Add => {
                 let a = v.pop().unwrap();
                 let b = v.pop().unwrap();
                 /* add some code here */
            },

            Mul => {
                 let a = v.pop().unwrap();
                 let b = v.pop().unwrap();
                 /* add some code here */
            },
        }
    }
    v.pop().unwrap()
}

fn main() {

    assert_eq!(eval(vec![Num(10)]), 10);
    assert_eq!(eval(vec![Num(10), Num(20), Add]), 30);
    assert_eq!(eval(vec![Num(10), Num(20), Mul]), 200);
    assert_eq!(eval(vec![Num(2), Num(3), Num(4), Add, Mul]), 14);
    assert_eq!(eval(vec![Num(2), Num(3), Add, Num(4), Mul]), 20);

    println!("\nAll tests passed!!\n");
   
    //submit::submit(Question(13), Score(10));
}
