fn main() {
    let some_value = Some(3);

    if let Some(x) = some_value {
        println!("{}", x);
    }
}
