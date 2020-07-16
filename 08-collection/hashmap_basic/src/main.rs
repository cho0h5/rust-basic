use std::collections::HashMap;

fn main() {
//    //let field_name = String::from("Favorite color");
//    let field_name = "Favorite color";
//    let field_value = String::from("Blue");
//
//    let mut map = HashMap::new();
//    map.insert(field_name, field_value);
//
//    println!("{:?}", map);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 100);

//    let score = scores.get("Blue");
//    println!("{}", score.unwrap());
    
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    scores.entry("Yellow".to_string()).or_insert(5);
    scores.entry("Blue".to_string()).or_insert(5);
    scores.entry("Green".to_string()).or_insert(5);

    println!("{:#?}", scores);
}
