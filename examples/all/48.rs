
fn identity<T>(x: T) -> T {
    x
}

fn main() {
    println!("{}", identity(10));
    println!("{}", identity(10.2));
    println!("{}", identity(true));
    println!("{}", identity("hello"));
    println!("{:?}", identity((1,2.3)));
}
