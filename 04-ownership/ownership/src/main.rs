fn main() {
    let s = "hello";
    println!("{}", s);

    let mut s = String::from("Hello");
    println!("{}", s);

    s.push_str(", world!");
    println!("{}", s);

    s = String::from("new string");
    println!("{}", s);
    

    let t = s;
    //println!("{}", s);
    println!("t: {}", t);

    let u = t.clone();
    println!("u: {}", u);
    println!("t: {}", t);
}
