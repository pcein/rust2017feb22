
// Software to control a chemical reactor

// If temperature goes above 100 degree Fahrenheit,
// Boom Boom Boom !!!

// Check whether temp is above explosion
// threshold.

fn above_explosion_threshold(tmp: f64) -> bool {
    
    tmp > 100.0 // expression is true if tmp is above 100

}

fn main () {

    let temp_celsius = 90.0;
    
    // poor programmer doesn't realize that
    // the unit used by the function is Fahrenheit
    // and NOT Celsius ...

    // 90 degree Celsius is 194 degree Fahrenheit,
    // very much ABOVE the explosion threshold!

    if above_explosion_threshold(temp_celsius) {
        println!("\nDANGER ... TAKE ACTION TO REDUCE TEMPERATURE ..\n");
    } else {
        println!("\nTemperature is under control, no problems!!\n");
    }

    // True story: https://en.wikipedia.org/wiki/Mars_Climate_Orbiter
    // Error in trajectory calculation because of software producing
    // output in non-SI units.
}
                


