fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    //let r3 = &mut s;
    //r3.push_str(", world!");

    println!("r1: {}", r1);
    println!("r2: {}", r2);
    //println!("r3: {}", r3);
}
