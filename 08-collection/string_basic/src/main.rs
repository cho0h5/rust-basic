fn main() {
//    let s1 = "Hello, ".to_string();
//    let s2 = "world!".to_string();
//    let s3 = &s1 + s2;
//
//    //println!("{}", s1);
//    println!("{}", s2);
//    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toc");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s);
}
