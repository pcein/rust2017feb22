
/* assertions */

fn is_delta_one(x: i32, y: i32) -> bool {
   (x - y) == 1
}

fn main() {

    assert_eq!(is_delta_one(11, 10), true);

    println!("\ntest passed\n");
    

}
