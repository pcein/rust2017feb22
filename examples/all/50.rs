
/* local variables, stack */

fn fun() {
    let x = 10; // space allocated on stack
    
    let y = 20; // space allocated on stack

    // space occupied by x and y automatically de-allocated   
}

fn main() {

    fun();

}
