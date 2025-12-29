// Celsius to Fahrenheit -> Work with formulas and number types.

fn main() {
    let temp_c = 60.0;
    let temp_f = 120.0;

    println!("{}C is {:.2}F", temp_c, celsius_to_fahrenheit(temp_c));
    println!("{}F is {:.2}C", temp_f, fahrenheit_to_celsius(temp_f));
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}