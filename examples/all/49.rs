
fn swap<T1,T2>(x: (T1, T2)) -> (T2,T1) {
    (x.1, x.0)
}

fn main() {
    println!("{:?}", swap((1,2.3)));
    println!("{:?}", swap(('A',"hello")));
}
