#[macro_use] extern crate scan_fmt;
use std::collections::HashMap;

fn main() {
    let mut company = HashMap::new();

    let q1 = String::from("Add Sally to Engineering");
    let q2 = String::from("Add Amir to Sales");
    let q3 = String::from("Add Youngho to Engineering");
    let q4 = String::from("Add Seok to Engineering");

    query(&q1, &mut company);
    query(&q2, &mut company);
    query(&q3, &mut company);
    query(&q4, &mut company);

    let q5 = String::from("Lookup Engineering");
    let q6 = String::from("Lookup Sales");

    query(&q5, &mut company);
    query(&q6, &mut company);
}

fn query(q: &String, company: &mut HashMap<String, String>) {
    if let Ok((name, department)) = scan_fmt!(q, "Add {} to {}", String, String) {
        company.insert(name, department);
    }

    if let Ok(department) = scan_fmt!(q, "Lookup {}", String) {
        for (key, value) in company {
            if *value == department {
                println!("{}: {}", key, value);
            }
        }
    }
}
