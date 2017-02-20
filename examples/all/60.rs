
fn main() {
    let mut v1 = vec![1, 2, 3];

    let v2 = &mut v1; // borrowed (mutably)

    // can't borrow mutably or immutably in the 
    // prescence of another mutable borrow!

    let v3 = &v1; 
}


