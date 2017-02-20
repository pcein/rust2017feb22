

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

    // Imagine we have this scenario: "v" is going out
    // of scope, so the space allocated for the vector
    // is once again freed. This is a bug - it should not
    // happen. And it will NOT happen because of two VERY
    // important ideas in Rust: Ownership and Move Semantics

    // Next program illustrates the concept.

}
