
fn foo() {
    
    // space for the actual contents of the vector 
    // is allocated on the heap.
    // The stack holds a pointer to the vector (plus
    // two other pieces of info: length and capacity)

    let v = vec![1,2,3];
    
    // when the variable "v" goes out of scope,
    // space allocated on the heap is automatically
    // de-allocated.

    // also check 51.c
}

fn main() {
    foo();
}
    


