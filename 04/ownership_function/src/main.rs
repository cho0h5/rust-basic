fn main() {
    let s = String::from("hello");

    let s = print_string(s);
    print_string(s);
}

fn print_string(s: String) -> String {
    println!("{}", s);
    s
}
