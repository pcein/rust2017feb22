

fn foo(v: Vec<i32>) {
    
    println!("{:?}", v);

    // "v" goes out of scope, space allocated
    // for the vector is freed.
}

fn main() {
    
    let v = vec![1, 2, 3];
    
    // The whole vector is NOT copied to "foo"
    // What gets copied to "v" in "foo" is simpy
    // the address of the memory block storing the
    // contents of the vector, plus two other pieces
    // of information: its length and capacity.

    foo(v);

    // The code will not compile
    // Will explain why in the class! (ownership/move)

    println!("{:?}", v);
}
