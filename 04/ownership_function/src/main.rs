fn main() {
    let s = String::from("hello");

    let (s, len) = len_string(s);
    println!("{} {}", s, len);
}

fn print_string(s: String) -> String {
    println!("{}", s);
    s
}

fn len_string(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
