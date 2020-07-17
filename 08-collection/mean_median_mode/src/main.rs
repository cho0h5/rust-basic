fn main() {
    let list = vec![1, 2340, 345, 1, 334, 13, 73, 6212, 34];

    println!("list : {:?}", list);

    println!("mean: {}", mean(&list));
    println!("median: {}", median(&list));
    println!("mode: {}", mode(&list));
}

fn mean(list: &Vec<i32>) -> f64 {
    let sum: i32 = list.iter().sum();
    let len = list.len();

    sum as f64 / len as f64
}

fn median(list: &Vec<i32>) -> i32 {
    let mut list_clone = list.clone();
    list_clone.sort();
    let len = list.len();

    list_clone[len/2]
}

fn mode(list: &Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::new();

    for e in list.iter() {
        let count = map.entry(e).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut max_value = 0;
    for (key, value) in &map {
        if value > &max_value {
            max = **key;
            max_value = *value;
        }
    }

    max
}
