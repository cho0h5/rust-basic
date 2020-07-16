fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = 50;
    println!("The value of x is: {}", x);
    let x = x * 2;
    println!("The value of x is: {}", x);
    
    const MAX_POINTS: u32 = 100000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);


    let spaces = "           ";
    println!("The value of spaces is: {}", spaces);
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);


    let _guess: u32 = "42".parse().expect("Not a number!");

    let _sum = 5.0 + 2.2;

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = ' ';
    println!("{} {} {}", c, z, heart_eyed_cat);

    let tup = (500, 6.4, 1);
    println!("tup: {:?}", tup);

    let x: (i32, f64, u8) = tup;

    println!("{}{}{}", x.0, x.1, x.2);
    

    let a = [1, 2, 3, 7, 1, 3, 8];
    let index = 10;
    println!("{}{}{}{}", a[0], a[1], a[index], a[6]);
}
