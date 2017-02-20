
fn sqr(x: i32) -> i32 {
    x*x
}

fn main() {
    
    let v1:Vec<i32> = (1..6).map(|x| x*x).collect();

    let v2:Vec<i32> = (1..6).map(sqr).collect();

    println!("v1 = {:?}, v2 = {:?}", v1, v2);

}
