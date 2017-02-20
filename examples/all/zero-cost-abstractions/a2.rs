
const N: u64 = 100000000;

fn main() {
    let r = (0..N).fold(0, |sum, i| sum+i);
    println!("{}", r);
}
