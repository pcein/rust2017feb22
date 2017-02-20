
/* Basic function definition */

fn is_delta_one(x: i32, y: i32) -> bool {
   (x - y) == 1
}

fn main() {

    println!("{}", is_delta_one(11, 10));
    println!("{}", is_delta_one(10, 11));

}
