fn main() {
    let celsius = 23.0;
    println!("fahrenheit: {}", c2f(celsius));
    println!("celsius: {}", f2c(c2f(celsius)));
}

fn c2f(temp: f64) -> f64 {
    temp * 1.8 + 32.0
}

fn f2c(temp: f64) -> f64 {
    (temp - 32.0) / 1.8
}
