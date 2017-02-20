
/* what happens if there is semi-colon 
 * on the last line of a function?
 */

fn sqr(x: i32) -> i32 {
    x * x;
}

fn main() {
    let r = sqr(10);
}
