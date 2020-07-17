fn main() {
    let numbers = vec![54, 21, 87, 23, 76, 123, 4];

    let largest = largest(&numbers);
    println!("The largest number is {}", largest);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
