fn main() {
    let number = 32;
    
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }


    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("number: {}", number);

    //loop {
    //    println!("again!");
    //}

    let mut number = 10;
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LISTOFF!!!");


    let a = [10, 20, 30, 50, 100];
    let mut index = 0;

    while index < 5 {
        println!("{}", a[index]);
        index = index + 1;
    }

    for number in (1..40).rev() {
        println!("{}!", number);
    }

    for number in a.iter() {
        println!("{}", number);
    }
}
