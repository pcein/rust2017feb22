
trait HasArea {
    fn area(&self) -> f64;
}

fn print_both_areas<T1:HasArea,T2:HasArea>(x:(T1,T2)) {

    println!("{},{}", x.0.area(), x.1.area());

}

fn main() {


}
