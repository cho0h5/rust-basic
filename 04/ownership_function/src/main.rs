fn main() {
    let s = String::from("hello");

    print_string(s);
    print_string(s);
}

fn print_string(s: String) {
    println!("{}", s);
}
