fn main() {
    let first = String::from("first");
    let apple = String::from("apple");
    let clock = String::from("clock");
    let among = String::from("among");

    println!("first: {}", pig_latin(&first));
    println!("apple: {}", pig_latin(&apple));
    println!("clock: {}", pig_latin(&clock));
    println!("among: {}", pig_latin(&among));
}

fn pig_latin(original_string: &String) -> String {
    let mut s = original_string.clone();

    let c = s.remove(0);

    if c == 'a'
        || c == 'e'
        || c == 'i'
        || c == 'o'
        || c == 'u' {
        s.insert(0, c);
        s.push_str("_hay");
    } else {
        s.push_str(&format!("_{}ay", c));
    }

    s
}
