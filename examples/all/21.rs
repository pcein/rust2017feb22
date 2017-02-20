
/* filter and collect */

fn main() {
    
    let v:Vec<i32> = (0 .. 10).filter(|x| x % 2 == 1).collect();

    println!("{:?}", v);
}

