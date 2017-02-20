
fn identity(x: i32) -> i32  {
    x
}

fn identity_float(x: f64) -> f64 {
    x
}

fn identity_bool(x: bool) -> bool {
    x
}

fn main() {
    println!("{}", identity(10));
    println!("{}", identity(2345));
    println!("{}", identity_float(1.2));
    println!("{}", identity_bool(true));
}
    
