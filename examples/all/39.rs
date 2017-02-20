
struct Celsius(f64);

struct Fahrenheit(f64);

fn above_explosion_threshold(tmp: Fahrenheit) -> bool {
    
    match tmp {
        Fahrenheit(t) => t > 100.0,
    }

}

fn main () {

    let temp_celsius = Celsius(90.0);
    
    if above_explosion_threshold(temp_celsius) {
        println!("\nDANGER ... TAKE ACTION TO REDUCE TEMPERATURE ..\n");
    } else {
        println!("\nTemperature is under control, no problems!!\n");
    }

}
