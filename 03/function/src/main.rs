fn main() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("x: {}", x);
    println!("y: {}", y);

    another_function(1217408, 8.0);
}

fn add1(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32, y: f64) {
    println!("x: {}", x);
    println!("y: {}", y);
}
